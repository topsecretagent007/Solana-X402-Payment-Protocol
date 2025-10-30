import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
  ConfirmOptions,
} from '@solana/web3.js';
import { serialize, deserialize } from 'borsh';

// Payment status enum
export enum PaymentStatus {
  Pending = 0,
  Completed = 1,
  Cancelled = 2,
}

// Helper function to convert status number to string
export function paymentStatusToString(status: PaymentStatus): string {
  switch (status) {
    case PaymentStatus.Pending:
      return 'Pending';
    case PaymentStatus.Completed:
      return 'Completed';
    case PaymentStatus.Cancelled:
      return 'Cancelled';
    default:
      return 'Unknown';
  }
}

// Payment account structure
export class Payment {
  payer: Uint8Array;
  recipient: Uint8Array;
  amount: bigint;
  payment_id: string;
  status: PaymentStatus;
  timestamp: bigint;

  constructor(fields: {
    payer: Uint8Array;
    recipient: Uint8Array;
    amount: bigint;
    payment_id: string;
    status: PaymentStatus;
    timestamp: bigint;
  }) {
    this.payer = fields.payer;
    this.recipient = fields.recipient;
    this.amount = fields.amount;
    this.payment_id = fields.payment_id;
    this.status = fields.status;
    this.timestamp = fields.timestamp;
  }

  // Convert Uint8Array to PublicKey
  getPayerPublicKey(): PublicKey {
    return new PublicKey(this.payer);
  }

  getRecipientPublicKey(): PublicKey {
    return new PublicKey(this.recipient);
  }

  // Convert timestamp to Date
  getTimestampDate(): Date {
    return new Date(Number(this.timestamp) * 1000);
  }
}

// Borsh schema for Payment
const PaymentSchema = new Map([
  [
    Payment,
    {
      kind: 'struct',
      fields: [
        ['payer', [32]],
        ['recipient', [32]],
        ['amount', 'u64'],
        ['payment_id', 'string'],
        ['status', 'u8'],
        ['timestamp', 'i64'],
      ],
    },
  ],
]);

export class PaymentProtocolClient {
  private connection: Connection;
  private programId: PublicKey;
  private confirmOptions: ConfirmOptions;

  constructor(
    connection: Connection,
    programId: PublicKey,
    confirmOptions: ConfirmOptions = { commitment: 'confirmed' }
  ) {
    this.connection = connection;
    this.programId = programId;
    this.confirmOptions = confirmOptions;
  }

  /**
   * Derive the payment account PDA
   */
  async getPaymentPDA(
    payer: PublicKey,
    paymentId: string
  ): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [
        Buffer.from('payment'),
        payer.toBuffer(),
        Buffer.from(paymentId),
      ],
      this.programId
    );
  }

  /**
   * Initialize a new payment
   */
  async initializePayment(
    payer: Keypair,
    recipient: PublicKey,
    amount: number,
    paymentId: string
  ): Promise<string> {
    try {
      // Validate inputs
      if (amount <= 0) {
        throw new Error('Payment amount must be greater than 0');
      }
      if (!paymentId || paymentId.length === 0) {
        throw new Error('Payment ID cannot be empty');
      }

      const [paymentPDA] = await this.getPaymentPDA(payer.publicKey, paymentId);

      // Check if payment account already exists
      const existingAccount = await this.connection.getAccountInfo(paymentPDA);
      if (existingAccount) {
        throw new Error(`Payment with ID "${paymentId}" already exists`);
      }

      // Instruction: 0 = InitializePayment
      const instructionData = Buffer.concat([
        Buffer.from([0]), // Instruction discriminator
        this.serializeU64(amount),
        this.serializeString(paymentId),
      ]);

      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: paymentPDA, isSigner: false, isWritable: true },
          { pubkey: recipient, isSigner: false, isWritable: false },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId: this.programId,
        data: instructionData,
      });

      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [payer],
        this.confirmOptions
      );

      console.log(`✅ Payment initialized: ${signature}`);
      console.log(`   Payment ID: ${paymentId}`);
      console.log(`   Amount: ${amount / LAMPORTS_PER_SOL} SOL`);
      console.log(`   PDA: ${paymentPDA.toBase58()}`);

      return signature;
    } catch (error) {
      console.error('❌ Failed to initialize payment:', error);
      throw error;
    }
  }

  /**
   * Complete a payment (transfer funds)
   */
  async completePayment(
    payer: Keypair,
    recipient: PublicKey,
    paymentId: string
  ): Promise<string> {
    try {
      const [paymentPDA] = await this.getPaymentPDA(payer.publicKey, paymentId);

      // Verify payment exists and is pending
      const payment = await this.getPayment(payer.publicKey, paymentId);
      if (!payment) {
        throw new Error(`Payment with ID "${paymentId}" not found`);
      }
      if (payment.status !== PaymentStatus.Pending) {
        throw new Error(
          `Payment is ${paymentStatusToString(payment.status)}, cannot complete`
        );
      }

      // Instruction: 1 = CompletePayment
      const instructionData = Buffer.from([1]);

      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: paymentPDA, isSigner: false, isWritable: true },
          { pubkey: recipient, isSigner: false, isWritable: true },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId: this.programId,
        data: instructionData,
      });

      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [payer],
        this.confirmOptions
      );

      console.log(`✅ Payment completed: ${signature}`);
      console.log(`   Amount: ${Number(payment.amount) / LAMPORTS_PER_SOL} SOL transferred`);
      return signature;
    } catch (error) {
      console.error('❌ Failed to complete payment:', error);
      throw error;
    }
  }

  /**
   * Cancel a payment
   */
  async cancelPayment(payer: Keypair, paymentId: string): Promise<string> {
    try {
      const [paymentPDA] = await this.getPaymentPDA(payer.publicKey, paymentId);

      // Verify payment exists and is pending
      const payment = await this.getPayment(payer.publicKey, paymentId);
      if (!payment) {
        throw new Error(`Payment with ID "${paymentId}" not found`);
      }
      if (payment.status !== PaymentStatus.Pending) {
        throw new Error(
          `Payment is ${paymentStatusToString(payment.status)}, cannot cancel`
        );
      }

      // Instruction: 2 = CancelPayment
      const instructionData = Buffer.from([2]);

      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: payer.publicKey, isSigner: true, isWritable: true },
          { pubkey: paymentPDA, isSigner: false, isWritable: true },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId: this.programId,
        data: instructionData,
      });

      const transaction = new Transaction().add(instruction);
      const signature = await sendAndConfirmTransaction(
        this.connection,
        transaction,
        [payer],
        this.confirmOptions
      );

      console.log(`✅ Payment cancelled: ${signature}`);
      return signature;
    } catch (error) {
      console.error('❌ Failed to cancel payment:', error);
      throw error;
    }
  }

  /**
   * Get payment account info
   */
  async getPayment(payer: PublicKey, paymentId: string): Promise<Payment | null> {
    try {
      const [paymentPDA] = await this.getPaymentPDA(payer, paymentId);
      const accountInfo = await this.connection.getAccountInfo(paymentPDA);

      if (!accountInfo) {
        return null;
      }

      const payment = deserialize(PaymentSchema, Payment, accountInfo.data);
      return payment;
    } catch (error) {
      console.error('❌ Failed to get payment:', error);
      return null;
    }
  }

  /**
   * Check if a payment exists
   */
  async paymentExists(payer: PublicKey, paymentId: string): Promise<boolean> {
    const payment = await this.getPayment(payer, paymentId);
    return payment !== null;
  }

  /**
   * Get payment status
   */
  async getPaymentStatus(
    payer: PublicKey,
    paymentId: string
  ): Promise<PaymentStatus | null> {
    const payment = await this.getPayment(payer, paymentId);
    return payment ? payment.status : null;
  }

  // Helper methods
  private serializeU64(value: number): Buffer {
    const buffer = Buffer.alloc(8);
    buffer.writeBigUInt64LE(BigInt(value));
    return buffer;
  }

  private serializeString(value: string): Buffer {
    const stringBytes = Buffer.from(value, 'utf8');
    const lengthBuffer = Buffer.alloc(4);
    lengthBuffer.writeUInt32LE(stringBytes.length);
    return Buffer.concat([lengthBuffer, stringBytes]);
  }
}

