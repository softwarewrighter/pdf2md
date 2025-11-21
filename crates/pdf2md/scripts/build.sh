#!/usr/bin/env bash
#
# Build script for pdf2md
# Runs all quality checks and builds the project

set -e  # Exit on error

echo "=== pdf2md Build Script ==="
echo

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Step 1: Format code
print_step "Step 1: Formatting code with cargo fmt"
if cargo fmt --check; then
    print_success "Code formatting check passed"
else
    print_error "Code formatting check failed"
    echo "Run 'cargo fmt' to fix formatting issues"
    exit 1
fi
echo

# Step 2: Lint with clippy
print_step "Step 2: Linting with cargo clippy"
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy linting passed"
else
    print_error "Clippy linting failed"
    echo "Fix all clippy warnings before proceeding"
    exit 1
fi
echo

# Step 3: Build in debug mode
print_step "Step 3: Building in debug mode"
if cargo build; then
    print_success "Debug build successful"
else
    print_error "Debug build failed"
    exit 1
fi
echo

# Step 4: Run tests
print_step "Step 4: Running tests"
if cargo test; then
    print_success "All tests passed"
else
    print_error "Tests failed"
    exit 1
fi
echo

# Step 5: Build in release mode
print_step "Step 5: Building in release mode"
if cargo build --release; then
    print_success "Release build successful"
else
    print_error "Release build failed"
    exit 1
fi
echo

# Step 6: Generate documentation
print_step "Step 6: Generating documentation"
if cargo doc --no-deps; then
    print_success "Documentation generated"
else
    print_error "Documentation generation failed"
    exit 1
fi
echo

# Summary
echo "=== Build Complete ==="
echo -e "${GREEN}All quality checks passed!${NC}"
echo
echo "Binary location: target/release/pdf2md"
echo "Documentation: target/doc/pdf2md/index.html"
echo
echo "To install: cargo install --path ."
