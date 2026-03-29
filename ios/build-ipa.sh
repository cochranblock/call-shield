#!/bin/bash
set -euo pipefail

# Build Call Shield iOS static library + IPA
# Run from: call-shield/ios/

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Building Rust static library for iOS ==="
cargo build --target aarch64-apple-ios --release
echo "  Built: target/aarch64-apple-ios/release/libcall_shield_ios.a"

STATIC_LIB="$SCRIPT_DIR/target/aarch64-apple-ios/release/libcall_shield_ios.a"

if [ ! -f "$STATIC_LIB" ]; then
    echo "ERROR: Static library not found at $STATIC_LIB"
    exit 1
fi

echo ""
echo "=== Static library ready ==="
echo "  Size: $(wc -c < "$STATIC_LIB") bytes"
echo ""
echo "Next steps (requires Xcode):"
echo "  1. Open ios/CallShield.xcodeproj (create via Xcode)"
echo "  2. Add libcall_shield_ios.a to Link Binary With Libraries"
echo "  3. Add bridging header path if needed"
echo "  4. Build: xcodebuild -scheme CallShield -sdk iphoneos -configuration Release archive"
echo ""
echo "Or generate project with xcodegen:"
echo "  brew install xcodegen"
echo "  cd ios && xcodegen generate"
echo "  xcodebuild -project CallShield.xcodeproj -scheme CallShield -sdk iphoneos archive"
