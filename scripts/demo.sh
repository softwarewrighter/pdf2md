#!/usr/bin/env bash
#
# Demo script for pdf2md
# Demonstrates the various features of the tool

set -e  # Exit on error

echo "=== pdf2md Demo Script ==="
echo

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_header() {
    echo
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_command() {
    echo -e "${YELLOW}\$ $1${NC}"
}

# Check if pdf2md is built
if [ ! -f "target/release/pdf2md" ] && [ ! -f "target/debug/pdf2md" ]; then
    echo "Error: pdf2md binary not found. Run ./scripts/build.sh first."
    exit 1
fi

# Use release build if available, otherwise debug
if [ -f "target/release/pdf2md" ]; then
    PDF2MD="./target/release/pdf2md"
else
    PDF2MD="./target/debug/pdf2md"
fi

echo "Using binary: $PDF2MD"

# Create demo directory
DEMO_DIR="demo_output"
mkdir -p "$DEMO_DIR"

# Check if test PDF exists
TEST_PDF="tests/fixtures/sample.pdf"
if [ ! -f "$TEST_PDF" ]; then
    echo "Error: Test PDF not found at $TEST_PDF"
    echo "Please generate it first using the appropriate script."
    exit 1
fi

# Demo 1: Help
print_header "Demo 1: Display Help"
print_command "$PDF2MD --help"
$PDF2MD --help
echo

# Demo 2: Version
print_header "Demo 2: Display Version"
print_command "$PDF2MD --version"
$PDF2MD --version
echo

# Demo 3: Dry-run mode
print_header "Demo 3: Dry-run Mode (Preview PDF)"
print_command "$PDF2MD -i $TEST_PDF -o $DEMO_DIR/output.md --dry-run"
$PDF2MD -i "$TEST_PDF" -o "$DEMO_DIR/output.md" --dry-run
echo

# Demo 4: Basic conversion
print_header "Demo 4: Basic Conversion"
print_command "$PDF2MD -i $TEST_PDF -o $DEMO_DIR/basic_output.md"
$PDF2MD -i "$TEST_PDF" -o "$DEMO_DIR/basic_output.md"
echo -e "${GREEN}✓ Output written to $DEMO_DIR/basic_output.md${NC}"
echo
echo "First 10 lines of output:"
head -10 "$DEMO_DIR/basic_output.md" || cat "$DEMO_DIR/basic_output.md"
echo

# Demo 5: Verbose mode
print_header "Demo 5: Verbose Mode"
print_command "$PDF2MD -i $TEST_PDF -o $DEMO_DIR/verbose_output.md --verbose"
$PDF2MD -i "$TEST_PDF" -o "$DEMO_DIR/verbose_output.md" --verbose
echo

# Demo 6: Nested output directory
print_header "Demo 6: Nested Output Directory"
print_command "$PDF2MD -i $TEST_PDF -o $DEMO_DIR/nested/deep/output.md"
$PDF2MD -i "$TEST_PDF" -o "$DEMO_DIR/nested/deep/output.md"
echo -e "${GREEN}✓ Output written to $DEMO_DIR/nested/deep/output.md${NC}"
echo

# Demo 7: Short flags
print_header "Demo 7: Using Short Flags"
print_command "$PDF2MD -i $TEST_PDF -o $DEMO_DIR/short_flags.md -v -n"
$PDF2MD -i "$TEST_PDF" -o "$DEMO_DIR/short_flags.md" -v -n
echo

# Demo 8: Error handling - nonexistent file
print_header "Demo 8: Error Handling (Nonexistent File)"
print_command "$PDF2MD -i nonexistent.pdf -o $DEMO_DIR/error.md"
if $PDF2MD -i nonexistent.pdf -o "$DEMO_DIR/error.md" 2>&1; then
    echo "Unexpected success"
else
    echo -e "${GREEN}✓ Error handled correctly${NC}"
fi
echo

# Summary
print_header "Demo Complete"
echo "Demo outputs are in the '$DEMO_DIR' directory:"
echo
ls -lh "$DEMO_DIR"
echo
echo "You can view the generated markdown files:"
echo "  cat $DEMO_DIR/basic_output.md"
echo
echo -e "${GREEN}All demos completed successfully!${NC}"
