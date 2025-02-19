#!/bin/bash

# ==============================
# Configuration
# ==============================
ROOT_DIR="./"  # The script must be run from the root of rust-lab
TOPICS_DIR="./topics"
CARGO_TOML="./Cargo.toml"

# List of reserved names (Cargo keywords, system directories, etc.)
RESERVED_NAMES=("test" "tests" "src" "target" "examples" "benchmarks" "benches" "bin" "lib" "core" "cargo")

# ==============================
# Check execution context
# ==============================
if [ ! -f "$CARGO_TOML" ]; then
    echo "❌ Error: This script must be executed from the root of the Cargo workspace (where Cargo.toml is located)."
    exit 1
fi

# ==============================
# Ensure `[workspace]` exists in Cargo.toml
# ==============================
if ! grep -q "^\[workspace\]" "$CARGO_TOML"; then
    echo "🔧 Adding [workspace] section to Cargo.toml..."
    cat >> "$CARGO_TOML" <<EOL

[workspace]
members = []
EOL
fi

# ==============================
# Argument validation
# ==============================
if [ -z "$1" ]; then
    echo "❌ Error: You must provide an topic name."
    echo "Usage: ./create_topic.sh <topic_name>"
    exit 1
fi

TOPIC_NAME=$1
EXP_DIR="$TOPICS_DIR/$TOPIC_NAME"

# Check if the topic name is valid (only letters, numbers, and dashes)
if [[ ! "$TOPIC_NAME" =~ ^[a-zA-Z0-9_-]+$ ]]; then
    echo "❌ Error: The topic name must contain only letters, numbers, and dashes (-)."
    exit 1
fi

# Check if the topic name is reserved
for RESERVED in "${RESERVED_NAMES[@]}"; do
    if [[ "$TOPIC_NAME" == "$RESERVED" ]]; then
        echo "❌ Error: '$TOPIC_NAME' is a reserved name and cannot be used."
        exit 1
    fi
done

# Check if the project already exists
if [ -d "$EXP_DIR" ]; then
    echo "⚠️ Warning: The topic '$TOPIC_NAME' already exists."
    exit 0
fi

# ==============================
# Create the Cargo project
# ==============================
echo "📂 Creating topic: $TOPIC_NAME"
cargo new --vcs none "$EXP_DIR"

# Create `examples/` and `tests/` subdirectories
mkdir -p "$EXP_DIR/examples"
mkdir -p "$EXP_DIR/tests"

# Add a basic example file
cat > "$EXP_DIR/examples/basic.rs" <<EOL
fn main() {
    println!("Hello from the {} example!", env!("CARGO_PKG_NAME"));
}
EOL

# Add a basic test file
cat > "$EXP_DIR/tests/basic_test.rs" <<EOL
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
EOL

echo "✅ topic '$TOPIC_NAME' successfully created and added to the workspace!"
exit 0
