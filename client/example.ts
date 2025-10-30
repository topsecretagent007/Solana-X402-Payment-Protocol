import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { PaymentProtocolClient, paymentStatusToString } from './payment-client';

/**
 * Solana X402 Payment Protocol - Example Usage
 * 
 * This example demonstrates:
 * 1. Initializing a payment
 * 2. Completing a payment
 * 3. Cancelling a payment
 * 4. Retrieving payment information
 * 
 * Contact: https://t.me/topsecretagent_007
 * GitHub: https://github.com/topsecretagent007/Solana-X402-Payment-Protocol
 */

async function main() {
  console.log('🚀 Solana X402 Payment Protocol - Example\n');

  // Connect to Solana devnet
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Replace with your deployed program ID
  const programId = new PublicKey('YOUR_PROGRAM_ID_HERE');

  // Create payer and recipient keypairs (for testing)
  const payer = Keypair.generate();
  const recipient = Keypair.generate();

  console.log('🔑 Payer:', payer.publicKey.toBase58());
  console.log('🔑 Recipient:', recipient.publicKey.toBase58());

  // Airdrop SOL to payer (devnet only)
  console.log('\n💰 Requesting airdrop...');
  const airdropSignature = await connection.requestAirdrop(
    payer.publicKey,
    2 * LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(airdropSignature);
  console.log('✅ Airdrop confirmed');

  // Initialize payment client
  const client = new PaymentProtocolClient(connection, programId);

  // Example 1: Initialize a payment
  console.log('\n📝 Example 1: Initialize Payment');
  const paymentId1 = 'PAYMENT-001';
  const amount1 = 0.5 * LAMPORTS_PER_SOL; // 0.5 SOL
  await client.initializePayment(payer, recipient.publicKey, amount1, paymentId1);

  // Example 2: Complete the payment
  console.log('\n✅ Example 2: Complete Payment');
  await client.completePayment(payer, recipient.publicKey, paymentId1);

  // Example 3: Initialize and cancel a payment
  console.log('\n📝 Example 3: Initialize and Cancel Payment');
  const paymentId2 = 'PAYMENT-002';
  const amount2 = 0.3 * LAMPORTS_PER_SOL;
  await client.initializePayment(payer, recipient.publicKey, amount2, paymentId2);

  console.log('\n❌ Cancelling payment...');
  await client.cancelPayment(payer, paymentId2);

  // Example 4: Get payment info
  console.log('\n📊 Example 4: Get Payment Info');
  const payment = await client.getPayment(payer.publicKey, paymentId1);
  if (payment) {
    console.log('Payment Details:');
    console.log('  Status:', paymentStatusToString(payment.status));
    console.log('  Amount:', Number(payment.amount) / LAMPORTS_PER_SOL, 'SOL');
    console.log('  Payment ID:', payment.payment_id);
    console.log('  Payer:', payment.getPayerPublicKey().toBase58());
    console.log('  Recipient:', payment.getRecipientPublicKey().toBase58());
    console.log('  Timestamp:', payment.getTimestampDate().toISOString());
  }

  // Example 5: Check payment existence and status
  console.log('\n🔍 Example 5: Check Payment Status');
  const exists = await client.paymentExists(payer.publicKey, paymentId1);
  console.log(`Payment ${paymentId1} exists:`, exists);
  
  const status = await client.getPaymentStatus(payer.publicKey, paymentId1);
  if (status !== null) {
    console.log(`Payment ${paymentId1} status:`, paymentStatusToString(status));
  }

  console.log('\n🎉 All examples completed!');
  console.log('\n📞 Contact: https://t.me/topsecretagent_007');
  console.log('🔗 GitHub: https://github.com/topsecretagent007/Solana-X402-Payment-Protocol');
}

main().catch((error) => {
  console.error('Error:', error);
  process.exit(1);
});

