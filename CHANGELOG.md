# Changelog

All notable changes to the Solana X402 Payment Protocol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-30

### Added
- Clock sysvar integration for accurate timestamp tracking
- `invoke_signed` for secure PDA operations
- Comprehensive error handling with detailed error messages
- Input validation for payment amounts and IDs
- TypeScript helper methods: `paymentExists()`, `getPaymentStatus()`
- Export of `PaymentStatus` enum and `paymentStatusToString()` helper
- Helper methods in Payment class: `getPayerPublicKey()`, `getRecipientPublicKey()`, `getTimestampDate()`
- Comprehensive integration tests for all payment operations
- `.gitignore` file with comprehensive rules
- MIT License file
- Enhanced example with all features demonstrated
- Better TypeScript type safety throughout

### Changed
- Updated to Solana 2.0 dependencies
- Updated Borsh to 1.5 for Rust and 2.0.0 for TypeScript
- Updated @solana/web3.js to 1.95.3
- Enhanced README with contact information, badges, and recent updates section
- Improved error messages in all Rust functions
- Payment client now validates payment state before operations
- Version bumped to 0.2.0 in both Cargo.toml and package.json

### Fixed
- Recipient validation in `complete_payment` function
- Proper confirmation options in TypeScript client
- Balance check before payment completion

### Security
- Added signed PDA invocation for enhanced security
- Comprehensive validation of all payment operations
- Better error handling to prevent unexpected states

## [0.1.0] - 2025-10-29

### Added
- Initial release of Solana X402 Payment Protocol
- Basic payment initialization functionality
- Payment completion with fund transfer
- Payment cancellation capability
- TypeScript client library
- Basic documentation
- Example usage code

[0.2.0]: https://github.com/topsecretagent007/Solana-X402-Payment-Protocol/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/topsecretagent007/Solana-X402-Payment-Protocol/releases/tag/v0.1.0

