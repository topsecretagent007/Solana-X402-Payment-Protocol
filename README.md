# Solana X402 Payment Protocol

> A simple, secure payment protocol smart contract on Solana blockchain for handling escrow payments and transfers.

[![GitHub](https://img.shields.io/badge/GitHub-topsecretagent007-blue?logo=github)](https://github.com/topsecretagent007/Solana-X402-Payment-Protocol)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Telegram](https://img.shields.io/badge/Telegram-@topsecretagent__007-blue?logo=telegram)](https://t.me/topsecretagent_007)

## ✨ Features

- 💰 **Initialize Payment** - Create payment with unique ID
- ✅ **Complete Payment** - Transfer SOL from payer to recipient
- ❌ **Cancel Payment** - Cancel pending payment
- 📊 **Payment Tracking** - Track status (Pending, Completed, Cancelled)
- 🔒 **Secure** - PDA-based with proper validation
- ⏰ **Timestamped** - Automatic timestamp tracking

## 🚀 Quick Start

### 1️⃣ Install Dependencies
```bash
npm install
```

### 2️⃣ Build the Program
```bash
cargo build-bpf
```

### 3️⃣ Deploy to Devnet
```bash
# Configure Solana CLI
solana config set --url https://api.devnet.solana.com

# Airdrop SOL
solana airdrop 2

# Deploy
solana program deploy target/deploy/solana_x402_payment_protocol.so
```

### 4️⃣ Update Your Program ID
Replace `YOUR_PROGRAM_ID_HERE` in `client/example.ts` with your deployed program ID.

### 5️⃣ Run Example
```bash
npm run example
```

## 📦 Installation

**Requirements:**
- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Node.js](https://nodejs.org/) v16+

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Clone repository
git clone https://github.com/topsecretagent007/Solana-X402-Payment-Protocol.git
cd Solana-X402-Payment-Protocol

# Install dependencies
npm install
```

## 💻 Usage

```typescript
import { PaymentProtocolClient } from './client/payment-client';
import { Connection, Keypair, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
const programId = new PublicKey('YOUR_PROGRAM_ID');
const client = new PaymentProtocolClient(connection, programId);

// Initialize payment
await client.initializePayment(payer, recipient, 0.5 * LAMPORTS_PER_SOL, 'PAYMENT-001');

// Complete payment
await client.completePayment(payer, recipient, 'PAYMENT-001');

// Cancel payment
await client.cancelPayment(payer, 'PAYMENT-001');

// Get payment info
const payment = await client.getPayment(payer.publicKey, 'PAYMENT-001');
```

## 🔧 API Reference

| Method | Description |
|--------|-------------|
| `initializePayment(payer, recipient, amount, paymentId)` | Create a new payment |
| `completePayment(payer, recipient, paymentId)` | Complete and transfer funds |
| `cancelPayment(payer, paymentId)` | Cancel a pending payment |
| `getPayment(payer, paymentId)` | Get payment details |
| `paymentExists(payer, paymentId)` | Check if payment exists |
| `getPaymentStatus(payer, paymentId)` | Get payment status |

## 📊 Payment Status

| Status | Value | Description |
|--------|-------|-------------|
| **Pending** | 0 | Payment created, waiting for action |
| **Completed** | 1 | Funds transferred to recipient |
| **Cancelled** | 2 | Payment cancelled, no transfer |

## 🛠️ Development

```bash
# Run tests
cargo test-bpf

# Build for production
cargo build-bpf --release

# Run example
npm run example

# Compile TypeScript
npm run compile
```

## 📚 Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- [Deployment Guide](DEPLOYMENT.md) - Complete deployment instructions  
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Changelog](CHANGELOG.md) - Version history

## 🔒 Security

⚠️ **Important**: This is for educational purposes. Before production:
- Conduct a professional security audit
- Test thoroughly on devnet
- Implement additional validation as needed
- Consider timeout mechanisms

## 📞 Contact & Support

- **GitHub**: [topsecretagent007](https://github.com/topsecretagent007)
- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
