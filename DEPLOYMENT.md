# Deployment Guide

Complete guide for deploying the Solana X402 Payment Protocol smart contract.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Building](#building)
- [Testing](#testing)
- [Deployment to Devnet](#deployment-to-devnet)
- [Deployment to Mainnet](#deployment-to-mainnet)
- [Verification](#verification)
- [Using the Client](#using-the-client)

## Prerequisites

Before deploying, ensure you have:

### Required Software
- **Rust** (latest stable version)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Solana CLI** (v2.0 or higher)
  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
  ```

- **Node.js** (v16 or higher)
  ```bash
  # Download from https://nodejs.org/
  ```

### Verify Installations
```bash
rustc --version
solana --version
node --version
npm --version
```

## Setup

### 1. Clone the Repository
```bash
git clone https://github.com/topsecretagent007/Solana-X402-Payment-Protocol.git
cd Solana-X402-Payment-Protocol
```

### 2. Install Dependencies

#### Rust Dependencies
```bash
cargo build
```

#### Node Dependencies
```bash
npm install
```

## Building

### Build for Development
```bash
cargo build-bpf
```

### Build for Production (Optimized)
```bash
cargo build-bpf --release
```

The compiled program will be located at:
- Dev: `target/deploy/solana_x402_payment_protocol.so`
- Release: `target/deploy/solana_x402_payment_protocol.so`

## Testing

### Run Rust Tests
```bash
cargo test-bpf
```

### Run TypeScript Examples (after deployment)
```bash
npm run example
```

## Deployment to Devnet

### 1. Configure Solana CLI for Devnet
```bash
solana config set --url https://api.devnet.solana.com
```

### 2. Create or Use Existing Keypair
```bash
# Create a new keypair
solana-keygen new --outfile ~/.config/solana/devnet-keypair.json

# Or use existing keypair
solana config set --keypair ~/.config/solana/devnet-keypair.json
```

### 3. Get Your Address
```bash
solana address
```

### 4. Airdrop SOL for Deployment
```bash
# Request 2 SOL
solana airdrop 2

# Check balance
solana balance
```

If airdrop fails, use the [Solana Devnet Faucet](https://faucet.solana.com/).

### 5. Deploy the Program
```bash
solana program deploy target/deploy/solana_x402_payment_protocol.so
```

### 6. Save Your Program ID
After deployment, you'll see:
```
Program Id: <YOUR_PROGRAM_ID>
```

**Important**: Save this Program ID! You'll need it for the client.

## Deployment to Mainnet

⚠️ **WARNING**: Mainnet deployment costs real SOL and should only be done after thorough testing!

### 1. Configure for Mainnet
```bash
solana config set --url https://api.mainnet-beta.solana.com
```

### 2. Use Your Mainnet Keypair
```bash
solana config set --keypair ~/.config/solana/mainnet-keypair.json
```

### 3. Ensure Sufficient Balance
Deployment typically costs ~2-5 SOL depending on program size.

```bash
solana balance
```

### 4. Deploy to Mainnet
```bash
# Build optimized version first
cargo build-bpf --release

# Deploy
solana program deploy target/deploy/solana_x402_payment_protocol.so
```

### 5. Verify Deployment
```bash
solana program show <YOUR_PROGRAM_ID>
```

## Verification

### Check Program Info
```bash
solana program show <YOUR_PROGRAM_ID>
```

### View Program Logs
```bash
solana logs <YOUR_PROGRAM_ID>
```

## Using the Client

### 1. Update Program ID in Client
Edit `client/example.ts`:

```typescript
const programId = new PublicKey('YOUR_ACTUAL_PROGRAM_ID');
```

### 2. Run the Example
```bash
npm run example
```

### 3. Integrate into Your Project
```typescript
import { PaymentProtocolClient } from './client/payment-client';
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com');
const programId = new PublicKey('YOUR_PROGRAM_ID');
const client = new PaymentProtocolClient(connection, programId);

// Use the client
await client.initializePayment(payer, recipient, amount, paymentId);
```

## Upgrading the Program

Solana programs are upgradeable if the upgrade authority is retained.

### 1. Build New Version
```bash
cargo build-bpf --release
```

### 2. Upgrade Program
```bash
solana program deploy --program-id <PROGRAM_ID> target/deploy/solana_x402_payment_protocol.so
```

## Troubleshooting

### Deployment Fails with "Insufficient Funds"
- Airdrop more SOL (devnet) or add funds (mainnet)
- Check balance: `solana balance`

### "Program already deployed" Error
- Use `--program-id` flag to upgrade existing program
- Or deploy with a new keypair

### Build Errors
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build-bpf`

### Client Connection Issues
- Verify network URL is correct
- Check Solana cluster status: https://status.solana.com/

## Best Practices

1. **Always test on devnet first**
2. **Verify program behavior thoroughly**
3. **Keep your keypairs secure**
4. **Document your program ID**
5. **Consider program upgrades carefully**
6. **Monitor program logs after deployment**

## Support

- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
- **GitHub Issues**: [Report Issues](https://github.com/topsecretagent007/Solana-X402-Payment-Protocol/issues)

## Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Stack Exchange](https://solana.stackexchange.com/)

