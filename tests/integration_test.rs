use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum PaymentInstruction {
    InitializePayment { amount: u64, payment_id: String },
    CompletePayment,
    CancelPayment,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Cancelled,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Payment {
    pub payer: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub payment_id: String,
    pub status: PaymentStatus,
    pub timestamp: i64,
}

fn get_payment_pda(program_id: &Pubkey, payer: &Pubkey, payment_id: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"payment", payer.as_ref(), payment_id.as_bytes()],
        program_id,
    )
}

#[tokio::test]
async fn test_initialize_payment() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_x402_payment_protocol",
        program_id,
        processor!(solana_x402_payment_protocol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let recipient = Keypair::new();
    let payment_id = "TEST-001";
    let amount = 1_000_000_000; // 1 SOL

    let (payment_pda, _bump) = get_payment_pda(&program_id, &payer.pubkey(), payment_id);

    let instruction_data = PaymentInstruction::InitializePayment {
        amount,
        payment_id: payment_id.to_string(),
    };

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new_readonly(recipient.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: instruction_data.try_to_vec().unwrap(),
    };

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Transaction should succeed");

    // Verify payment account was created
    let payment_account = banks_client
        .get_account(payment_pda)
        .await
        .expect("get_account")
        .expect("payment account should exist");

    let payment = Payment::try_from_slice(&payment_account.data).unwrap();
    assert_eq!(payment.payer, payer.pubkey());
    assert_eq!(payment.recipient, recipient.pubkey());
    assert_eq!(payment.amount, amount);
    assert_eq!(payment.payment_id, payment_id);
    assert_eq!(payment.status, PaymentStatus::Pending);
}

#[tokio::test]
async fn test_complete_payment() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_x402_payment_protocol",
        program_id,
        processor!(solana_x402_payment_protocol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let recipient = Keypair::new();
    let payment_id = "TEST-002";
    let amount = 500_000_000; // 0.5 SOL

    let (payment_pda, _bump) = get_payment_pda(&program_id, &payer.pubkey(), payment_id);

    // Initialize payment first
    let init_instruction_data = PaymentInstruction::InitializePayment {
        amount,
        payment_id: payment_id.to_string(),
    };

    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new_readonly(recipient.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_instruction_data.try_to_vec().unwrap(),
    };

    let mut init_transaction = Transaction::new_with_payer(&[init_instruction], Some(&payer.pubkey()));
    init_transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(init_transaction).await.unwrap();

    // Get a new blockhash for the next transaction
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    // Complete payment
    let complete_instruction_data = PaymentInstruction::CompletePayment;

    let complete_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new(recipient.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: complete_instruction_data.try_to_vec().unwrap(),
    };

    let mut complete_transaction = Transaction::new_with_payer(&[complete_instruction], Some(&payer.pubkey()));
    complete_transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(complete_transaction).await;
    assert!(result.is_ok(), "Complete payment should succeed");

    // Verify payment status is completed
    let payment_account = banks_client
        .get_account(payment_pda)
        .await
        .expect("get_account")
        .expect("payment account should exist");

    let payment = Payment::try_from_slice(&payment_account.data).unwrap();
    assert_eq!(payment.status, PaymentStatus::Completed);
}

#[tokio::test]
async fn test_cancel_payment() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_x402_payment_protocol",
        program_id,
        processor!(solana_x402_payment_protocol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let recipient = Keypair::new();
    let payment_id = "TEST-003";
    let amount = 300_000_000; // 0.3 SOL

    let (payment_pda, _bump) = get_payment_pda(&program_id, &payer.pubkey(), payment_id);

    // Initialize payment first
    let init_instruction_data = PaymentInstruction::InitializePayment {
        amount,
        payment_id: payment_id.to_string(),
    };

    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new_readonly(recipient.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_instruction_data.try_to_vec().unwrap(),
    };

    let mut init_transaction = Transaction::new_with_payer(&[init_instruction], Some(&payer.pubkey()));
    init_transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(init_transaction).await.unwrap();

    // Get a new blockhash for the next transaction
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    // Cancel payment
    let cancel_instruction_data = PaymentInstruction::CancelPayment;

    let cancel_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: cancel_instruction_data.try_to_vec().unwrap(),
    };

    let mut cancel_transaction = Transaction::new_with_payer(&[cancel_instruction], Some(&payer.pubkey()));
    cancel_transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(cancel_transaction).await;
    assert!(result.is_ok(), "Cancel payment should succeed");

    // Verify payment status is cancelled
    let payment_account = banks_client
        .get_account(payment_pda)
        .await
        .expect("get_account")
        .expect("payment account should exist");

    let payment = Payment::try_from_slice(&payment_account.data).unwrap();
    assert_eq!(payment.status, PaymentStatus::Cancelled);
}

#[tokio::test]
async fn test_cannot_complete_cancelled_payment() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "solana_x402_payment_protocol",
        program_id,
        processor!(solana_x402_payment_protocol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let recipient = Keypair::new();
    let payment_id = "TEST-004";
    let amount = 200_000_000;

    let (payment_pda, _bump) = get_payment_pda(&program_id, &payer.pubkey(), payment_id);

    // Initialize payment
    let init_instruction_data = PaymentInstruction::InitializePayment {
        amount,
        payment_id: payment_id.to_string(),
    };

    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new_readonly(recipient.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_instruction_data.try_to_vec().unwrap(),
    };

    let mut init_transaction = Transaction::new_with_payer(&[init_instruction], Some(&payer.pubkey()));
    init_transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(init_transaction).await.unwrap();

    // Get new blockhash
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    // Cancel payment
    let cancel_instruction_data = PaymentInstruction::CancelPayment;
    let cancel_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: cancel_instruction_data.try_to_vec().unwrap(),
    };

    let mut cancel_transaction = Transaction::new_with_payer(&[cancel_instruction], Some(&payer.pubkey()));
    cancel_transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(cancel_transaction).await.unwrap();

    // Get new blockhash
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    // Try to complete cancelled payment - should fail
    let complete_instruction_data = PaymentInstruction::CompletePayment;
    let complete_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(payment_pda, false),
            AccountMeta::new(recipient.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: complete_instruction_data.try_to_vec().unwrap(),
    };

    let mut complete_transaction = Transaction::new_with_payer(&[complete_instruction], Some(&payer.pubkey()));
    complete_transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(complete_transaction).await;
    assert!(result.is_err(), "Should not be able to complete a cancelled payment");
}

