# Quick Start Guide

Get up and running with Solana X402 Payment Protocol in 5 minutes!

## ⚡ Super Quick Start

### 1. Install Prerequisites
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Node.js dependencies
npm install
```

### 2. Configure Solana for Devnet
```bash
solana config set --url https://api.devnet.solana.com
solana-keygen new  # Create a new keypair if you don't have one
solana airdrop 2   # Get some devnet SOL
```

### 3. Build the Program
```bash
cargo build-bpf
```

### 4. Deploy to Devnet
```bash
solana program deploy target/deploy/solana_x402_payment_protocol.so
```

**Save your Program ID!** It will be displayed after deployment.

### 5. Update Client Code
Edit `client/example.ts` and replace:
```typescript
const programId = new PublicKey('YOUR_PROGRAM_ID_HERE');
```

With your actual Program ID.

### 6. Run the Example
```bash
npm run example
```

## 📚 Basic Usage

### Initialize a Payment
```typescript
import { PaymentProtocolClient } from './client/payment-client';
import { Connection, Keypair, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
const programId = new PublicKey('YOUR_PROGRAM_ID');
const client = new PaymentProtocolClient(connection, programId);

const payer = Keypair.fromSecretKey(/* your secret key */);
const recipient = new PublicKey('recipient_address');
const amount = 0.5 * LAMPORTS_PER_SOL; // 0.5 SOL

await client.initializePayment(payer, recipient, amount, 'PAYMENT-001');
```

### Complete a Payment
```typescript
await client.completePayment(payer, recipient, 'PAYMENT-001');
```

### Cancel a Payment
```typescript
await client.cancelPayment(payer, 'PAYMENT-001');
```

### Get Payment Info
```typescript
const payment = await client.getPayment(payer.publicKey, 'PAYMENT-001');
console.log('Status:', payment.status);
console.log('Amount:', Number(payment.amount) / LAMPORTS_PER_SOL, 'SOL');
```

## 🎯 Common Use Cases

### 1. Escrow Service
```typescript
// Buyer creates payment
await client.initializePayment(buyer, seller, amount, orderId);

// When goods delivered, buyer completes payment
await client.completePayment(buyer, seller, orderId);

// Or buyer cancels if something goes wrong
await client.cancelPayment(buyer, orderId);
```

### 2. Subscription Payment
```typescript
const subscriptionId = `SUB-${userId}-${Date.now()}`;
await client.initializePayment(subscriber, service, monthlyFee, subscriptionId);
await client.completePayment(subscriber, service, subscriptionId);
```

### 3. Conditional Payment
```typescript
// Create payment
await client.initializePayment(payer, recipient, amount, paymentId);

// Check some condition
if (conditionMet) {
  await client.completePayment(payer, recipient, paymentId);
} else {
  await client.cancelPayment(payer, paymentId);
}
```

## 🔧 Troubleshooting

### "Insufficient funds" Error
```bash
# Get more devnet SOL
solana airdrop 2
```

### "Program account not found"
- Make sure you deployed the program
- Verify you're using the correct Program ID
- Check you're on the right network (devnet/mainnet)

### "Payment already exists"
- Use a unique payment ID for each payment
- Or complete/cancel the existing payment first

### Connection Timeout
- Check your internet connection
- Try a different RPC endpoint
- Verify Solana network status: https://status.solana.com/

## 📖 Next Steps

- Read the [Full Documentation](README.md)
- Check the [Deployment Guide](DEPLOYMENT.md)
- Review [Contributing Guidelines](CONTRIBUTING.md)
- Explore the [Example Code](client/example.ts)

## 🆘 Need Help?

- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
- **GitHub Issues**: [Report a Problem](https://github.com/topsecretagent007/Solana-X402-Payment-Protocol/issues)

## ⚠️ Important Notes

1. **Never commit private keys** to version control
2. **Test thoroughly on devnet** before mainnet
3. **Save your Program ID** after deployment
4. **Keep your keypair secure**
5. **Audit before production use**

---

**Ready for Production?** Read the [Deployment Guide](DEPLOYMENT.md) for mainnet deployment.

**Want to Contribute?** Check out [Contributing Guidelines](CONTRIBUTING.md).

