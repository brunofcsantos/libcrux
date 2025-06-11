clear
(return 0 2>/dev/null) || { echo "This script must be sourced, not executed."; return; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
START_DIR=$PWD

cd $SCRIPT_DIR
echo $SCRIPT_DIR
cargo +nightly build --release


# Key generation
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-44 Key Generation/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-44-keygen-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-44-keygen-memory" > "$FOLDER/ml-dsa-44-keygen-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-44-keygen-memory.txt"

# Signing with key
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-44 Signing/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-44-sign-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-44-sign-memory" > "$FOLDER/ml-dsa-44-sign-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-44-sign-memory.txt"

# Preparing a signature and a pub key for testing the verification without generating them
../target/release/ml-dsa-44-prepare-verify

# Verifying key
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-44 Verification/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-44-verify-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-44-verify-memory" > "$FOLDER/ml-dsa-44-verify-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-44-verify-memory.txt"




# Key generation
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-65 Key Generation/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-65-keygen-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-65-keygen-memory" > "$FOLDER/ml-dsa-65-keygen-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-65-keygen-memory.txt"

# Signing with key
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-65 Signing/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-65-sign-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-65-sign-memory" > "$FOLDER/ml-dsa-65-sign-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-65-sign-memory.txt"

# Preparing a signature and a pub key for testing the verification without generating them
../target/release/ml-dsa-65-prepare-verify

# Verifying key
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-65 Verification/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-65-verify-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-65-verify-memory" > "$FOLDER/ml-dsa-65-verify-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-65-verify-memory.txt"




# Key generation
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-87 Key Generation/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-87-keygen-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-87-keygen-memory" > "$FOLDER/ml-dsa-87-keygen-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-87-keygen-memory.txt"

# Signing with key
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-87 Signing/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-87-sign-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-87-sign-memory" > "$FOLDER/ml-dsa-87-sign-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-87-sign-memory.txt"

# Preparing a signature and a pub key for testing the verification without generating them
../target/release/ml-dsa-87-prepare-verify

# Verifying key
FOLDER="$SCRIPT_DIR/../target/criterion/ML-DSA-87 Verification/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/ml-dsa-87-verify-memory.txt" "$SCRIPT_DIR/../target/release/ml-dsa-87-verify-memory" > "$FOLDER/ml-dsa-87-verify-memory.txt"
echo "Wrote to $FOLDER/ml-dsa-87-verify-memory.txt"