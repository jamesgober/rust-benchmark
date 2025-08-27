#!/bin/bash
# Setup pre-commit hooks for the benchmark project

echo "Setting up pre-commit hooks..."

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook for Rust benchmark library

set -e

echo "Running pre-commit checks..."

# Format check
echo "Checking formatting..."
cargo fmt -- --check

# Clippy
echo "Running clippy..."
cargo clippy --all-features -- -D warnings

# Tests
echo "Running tests..."
cargo test --all-features

# Test with no default features
echo "Testing with no default features..."
cargo test --no-default-features

# Check documentation
echo "Checking documentation..."
cargo doc --all-features --no-deps

echo "All pre-commit checks passed!"
EOF

# Make hook executable
chmod +x .git/hooks/pre-commit

echo "Pre-commit hooks installed successfully!"
echo "To skip hooks temporarily, use: git commit --no-verify"