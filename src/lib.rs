use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

// Program entrypoint
entrypoint!(process_instruction);

// Payment Protocol Instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum PaymentInstruction {
    /// Initialize a new payment
    /// Accounts:
    /// 0. [signer] Payer account
    /// 1. [writable] Payment account (PDA)
    /// 2. [] Recipient account
    /// 3. [] System program
    InitializePayment { amount: u64, payment_id: String },

    /// Complete the payment (transfer funds)
    /// Accounts:
    /// 0. [signer] Payer account
    /// 1. [writable] Payment account (PDA)
    /// 2. [writable] Recipient account
    /// 3. [] System program
    CompletePayment,

    /// Cancel and refund the payment
    /// Accounts:
    /// 0. [signer] Payer account
    /// 1. [writable] Payment account (PDA)
    /// 2. [] System program
    CancelPayment,
}

// Payment account state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Payment {
    pub payer: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub payment_id: String,
    pub status: PaymentStatus,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Cancelled,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PaymentInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        PaymentInstruction::InitializePayment { amount, payment_id } => {
            msg!("Instruction: Initialize Payment");
            initialize_payment(program_id, accounts, amount, payment_id)
        }
        PaymentInstruction::CompletePayment => {
            msg!("Instruction: Complete Payment");
            complete_payment(program_id, accounts)
        }
        PaymentInstruction::CancelPayment => {
            msg!("Instruction: Cancel Payment");
            cancel_payment(program_id, accounts)
        }
    }
}

fn initialize_payment(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    payment_id: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let payment_account = next_account_info(account_info_iter)?;
    let recipient_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Verify payer is signer
    if !payer_account.is_signer {
        msg!("Error: Payer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify minimum payment amount
    if amount == 0 {
        msg!("Error: Payment amount must be greater than 0");
        return Err(ProgramError::InvalidArgument);
    }

    // Get current timestamp from Clock sysvar
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    // Create payment data
    let payment = Payment {
        payer: *payer_account.key,
        recipient: *recipient_account.key,
        amount,
        payment_id: payment_id.clone(),
        status: PaymentStatus::Pending,
        timestamp,
    };

    // Serialize payment data
    let payment_data = payment.try_to_vec()?;
    let data_len = payment_data.len();

    // Calculate rent
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(data_len);

    // Derive PDA
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[b"payment", payer_account.key.as_ref(), payment_id.as_bytes()],
        program_id,
    );

    if pda != *payment_account.key {
        msg!("Error: Invalid payment account PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    // Create payment account using invoke_signed
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"payment",
        payer_account.key.as_ref(),
        payment_id.as_bytes(),
        &[bump_seed],
    ]];

    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            payment_account.key,
            rent_lamports,
            data_len as u64,
            program_id,
        ),
        &[payer_account.clone(), payment_account.clone(), system_program.clone()],
        signer_seeds,
    )?;

    // Write payment data
    payment.serialize(&mut &mut payment_account.data.borrow_mut()[..])?;

    msg!(
        "Payment initialized: ID={}, Amount={}, Timestamp={}",
        payment_id,
        amount,
        timestamp
    );
    Ok(())
}

fn complete_payment(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let payment_account = next_account_info(account_info_iter)?;
    let recipient_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Verify payer is signer
    if !payer_account.is_signer {
        msg!("Error: Payer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify payment account ownership
    if payment_account.owner != program_id {
        msg!("Error: Invalid payment account owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize payment data
    let mut payment = Payment::try_from_slice(&payment_account.data.borrow())?;

    // Verify payer matches
    if payment.payer != *payer_account.key {
        msg!("Error: Payer does not match payment account");
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify recipient matches
    if payment.recipient != *recipient_account.key {
        msg!("Error: Recipient does not match payment account");
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify payment is pending
    if payment.status != PaymentStatus::Pending {
        msg!("Error: Payment is not in pending status");
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify payer has sufficient balance
    if payer_account.lamports() < payment.amount {
        msg!("Error: Insufficient funds in payer account");
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer funds to recipient
    invoke(
        &system_instruction::transfer(payer_account.key, recipient_account.key, payment.amount),
        &[payer_account.clone(), recipient_account.clone(), system_program.clone()],
    )?;

    // Update payment status with completion timestamp
    payment.status = PaymentStatus::Completed;
    let clock = Clock::get()?;
    payment.timestamp = clock.unix_timestamp;
    payment.serialize(&mut &mut payment_account.data.borrow_mut()[..])?;

    msg!(
        "Payment completed: Amount={} transferred to recipient {}",
        payment.amount,
        recipient_account.key
    );
    Ok(())
}

fn cancel_payment(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let payment_account = next_account_info(account_info_iter)?;

    // Verify payer is signer
    if !payer_account.is_signer {
        msg!("Error: Payer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify payment account ownership
    if payment_account.owner != program_id {
        msg!("Error: Invalid payment account owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize payment data
    let mut payment = Payment::try_from_slice(&payment_account.data.borrow())?;

    // Verify payer matches
    if payment.payer != *payer_account.key {
        msg!("Error: Payer does not match payment account");
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify payment is pending
    if payment.status != PaymentStatus::Pending {
        msg!("Error: Payment is not in pending status");
        return Err(ProgramError::InvalidAccountData);
    }

    // Update payment status with cancellation timestamp
    payment.status = PaymentStatus::Cancelled;
    let clock = Clock::get()?;
    payment.timestamp = clock.unix_timestamp;
    payment.serialize(&mut &mut payment_account.data.borrow_mut()[..])?;

    msg!("Payment cancelled: ID={} at timestamp={}", payment.payment_id, payment.timestamp);
    Ok(())
}

