# Contributing to Solana X402 Payment Protocol

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## 🤝 How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:
- Clear description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, Solana version, etc.)
- Relevant logs or error messages

### Suggesting Features

Feature suggestions are welcome! Please:
- Check if the feature has already been requested
- Clearly describe the feature and its use case
- Explain why it would be valuable to the project

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/topsecretagent007/Solana-X402-Payment-Protocol.git
   cd Solana-X402-Payment-Protocol
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Write clear, commented code
   - Follow existing code style
   - Add tests for new features
   - Update documentation as needed

4. **Test your changes**
   ```bash
   # Run Rust tests
   cargo test-bpf
   
   # Build the project
   cargo build-bpf
   
   # Run TypeScript examples
   npm run example
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

   Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` New feature
   - `fix:` Bug fix
   - `docs:` Documentation changes
   - `test:` Adding or updating tests
   - `refactor:` Code refactoring
   - `chore:` Maintenance tasks

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Provide a clear description of changes
   - Reference any related issues
   - Ensure all checks pass

## 📝 Code Style Guidelines

### Rust Code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting:
  ```bash
  cargo fmt
  ```
- Use `clippy` for linting:
  ```bash
  cargo clippy -- -D warnings
  ```
- Add comments for complex logic
- Keep functions focused and modular

### TypeScript Code
- Use TypeScript strict mode
- Follow consistent naming conventions:
  - `camelCase` for variables and functions
  - `PascalCase` for classes and types
- Add JSDoc comments for public APIs
- Use meaningful variable names

### Documentation
- Update README.md for user-facing changes
- Update CHANGELOG.md following Keep a Changelog format
- Add inline comments for complex code
- Include examples for new features

## 🧪 Testing

### Writing Tests
- Add tests for all new features
- Ensure tests cover edge cases
- Use descriptive test names
- Keep tests isolated and reproducible

### Running Tests
```bash
# Rust integration tests
cargo test-bpf

# Build verification
cargo build-bpf

# TypeScript compilation
npm run compile
```

## 🔒 Security

### Reporting Security Issues
**Do not open public issues for security vulnerabilities!**

Instead:
- Contact via Telegram: [@topsecretagent_007](https://t.me/topsecretagent_007)
- Provide detailed information about the vulnerability
- Wait for confirmation before public disclosure

### Security Best Practices
- Never commit private keys or sensitive data
- Review code for potential security issues
- Consider gas/compute unit optimization
- Validate all inputs thoroughly

## 📋 Development Workflow

1. **Check existing issues** - Avoid duplicate work
2. **Discuss major changes** - Open an issue first
3. **Keep PRs focused** - One feature/fix per PR
4. **Write tests** - Maintain code quality
5. **Update docs** - Keep documentation current
6. **Be responsive** - Address review feedback promptly

## 🎯 Priority Areas

Current priority areas for contributions:
- Additional test coverage
- Performance optimizations
- Documentation improvements
- Example implementations
- Security enhancements

## 💬 Communication

- **GitHub Issues**: Bug reports and feature requests
- **Pull Requests**: Code contributions and discussions
- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007) - Quick questions and support

## 📜 Code of Conduct

### Our Standards
- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism
- Focus on what's best for the project
- Show empathy towards others

### Unacceptable Behavior
- Harassment or discrimination
- Trolling or insulting comments
- Publishing others' private information
- Other unprofessional conduct

## 🙏 Recognition

Contributors will be:
- Listed in project documentation
- Credited in release notes
- Acknowledged in the community

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🎓 Learning Resources

New to Solana development? Check these out:
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

## ❓ Questions?

Don't hesitate to ask!
- Open a GitHub issue with the "question" label
- Reach out on Telegram: [@topsecretagent_007](https://t.me/topsecretagent_007)

Thank you for contributing to Solana X402 Payment Protocol! 🚀

