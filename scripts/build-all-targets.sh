#!/bin/bash
set -euo pipefail

# Build Call Shield for every supported target architecture.
# Outputs binaries to release/call-shield-<target>
#
# Usage:
#   ./scripts/build-all-targets.sh          # build all available targets
#   ./scripts/build-all-targets.sh local    # only targets that build natively
#   ./scripts/build-all-targets.sh cross    # only targets that need cross/SSH

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

RELEASE_DIR="$PROJECT_ROOT/release"
mkdir -p "$RELEASE_DIR"

BINARY_NAME="call-shield"
MODE="${1:-all}"
BUILT=0
FAILED=0

build_local() {
    local target="$1"
    echo "--- Building: $target (local) ---"
    if cargo build --release --target "$target" 2>&1; then
        cp "target/$target/release/$BINARY_NAME" "$RELEASE_DIR/$BINARY_NAME-$target"
        SIZE=$(wc -c < "$RELEASE_DIR/$BINARY_NAME-$target" | tr -d ' ')
        echo "  OK: $RELEASE_DIR/$BINARY_NAME-$target ($SIZE bytes)"
        BUILT=$((BUILT + 1))
    else
        echo "  FAILED: $target"
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

build_cross() {
    local target="$1"
    echo "--- Building: $target (cross) ---"
    if command -v cross &>/dev/null; then
        if cross build --release --target "$target" 2>&1; then
            cp "target/$target/release/$BINARY_NAME" "$RELEASE_DIR/$BINARY_NAME-$target"
            SIZE=$(wc -c < "$RELEASE_DIR/$BINARY_NAME-$target" | tr -d ' ')
            echo "  OK: $RELEASE_DIR/$BINARY_NAME-$target ($SIZE bytes)"
            BUILT=$((BUILT + 1))
        else
            echo "  FAILED: $target (cross)"
            FAILED=$((FAILED + 1))
        fi
    else
        echo "  SKIPPED: 'cross' not installed (cargo install cross)"
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

build_ssh() {
    local target="$1"
    local host="$2"
    echo "--- Building: $target (SSH → $host) ---"
    if ssh "$host" "source ~/.cargo/env 2>/dev/null; cd ~/call-shield && cargo build --release" 2>&1; then
        scp "$host:~/call-shield/target/release/$BINARY_NAME" "$RELEASE_DIR/$BINARY_NAME-$target"
        SIZE=$(wc -c < "$RELEASE_DIR/$BINARY_NAME-$target" | tr -d ' ')
        echo "  OK: $RELEASE_DIR/$BINARY_NAME-$target ($SIZE bytes)"
        BUILT=$((BUILT + 1))
    else
        echo "  FAILED: $target via $host"
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

sync_to_nodes() {
    echo "=== Syncing source to build nodes ==="
    for host in st gd lf bt; do
        echo "  → $host"
        rsync -az --exclude target --exclude .git "$PROJECT_ROOT/" "$host:~/call-shield/" 2>/dev/null || true
    done
    echo ""
}

echo "============================================"
echo "  Call Shield — Multi-Architecture Build"
echo "============================================"
echo ""

# --- Native targets (Mac Mini) ---
if [ "$MODE" = "all" ] || [ "$MODE" = "local" ]; then
    echo "=== Native targets (macOS) ==="

    # macOS ARM (native)
    build_local "aarch64-apple-darwin"

    # macOS Intel (cross on ARM Mac)
    build_local "x86_64-apple-darwin"

    # iOS (static lib — separate crate)
    echo "--- Building: aarch64-apple-ios (local, staticlib) ---"
    if [ -f "$PROJECT_ROOT/ios/Cargo.toml" ]; then
        (cd "$PROJECT_ROOT/ios" && cargo build --release --target aarch64-apple-ios 2>&1) && {
            cp "$PROJECT_ROOT/ios/target/aarch64-apple-ios/release/libcall_shield_ios.a" \
               "$RELEASE_DIR/libcall_shield_ios-aarch64-apple-ios.a"
            SIZE=$(wc -c < "$RELEASE_DIR/libcall_shield_ios-aarch64-apple-ios.a" | tr -d ' ')
            echo "  OK: libcall_shield_ios-aarch64-apple-ios.a ($SIZE bytes)"
            BUILT=$((BUILT + 1))
        } || {
            echo "  FAILED: aarch64-apple-ios"
            FAILED=$((FAILED + 1))
        }
    fi
    echo ""

    # WASM
    echo "--- Building: wasm32-unknown-unknown (local) ---"
    if rustup target list --installed | grep -q wasm32-unknown-unknown; then
        build_local "wasm32-unknown-unknown"
    else
        echo "  SKIPPED: rustup target add wasm32-unknown-unknown"
        FAILED=$((FAILED + 1))
    fi
fi

# --- Cross-compiled / SSH targets ---
if [ "$MODE" = "all" ] || [ "$MODE" = "cross" ]; then
    echo "=== Cross-compiled targets ==="

    # Sync source to build nodes
    sync_to_nodes

    # Linux x86_64 (build on st)
    build_ssh "x86_64-unknown-linux-gnu" "st"

    # Linux x86_64 (also available on gd, lf)
    # build_ssh "x86_64-unknown-linux-gnu" "gd"

    # Linux ARM 64-bit (Raspberry Pi 4/5, AWS Graviton)
    build_cross "aarch64-unknown-linux-gnu"

    # Linux ARM 32-bit (older RPi, IoT)
    build_cross "armv7-unknown-linux-gnueabihf"

    # RISC-V 64-bit
    build_cross "riscv64gc-unknown-linux-gnu"

    # Windows (MinGW cross)
    build_cross "x86_64-pc-windows-gnu"

    # FreeBSD
    build_cross "x86_64-unknown-freebsd"

    # IBM POWER (government mainframes)
    build_cross "powerpc64le-unknown-linux-gnu"

    # Android ARM (native lib)
    build_cross "aarch64-linux-android"
fi

echo "============================================"
echo "  Results: $BUILT built, $FAILED failed/skipped"
echo "============================================"
echo ""
echo "Binaries in: $RELEASE_DIR/"
ls -lh "$RELEASE_DIR/" 2>/dev/null || true
