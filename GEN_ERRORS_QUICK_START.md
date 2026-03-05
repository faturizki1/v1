# Panduan Cepat gen_errors

Script untuk otomatis membuat 5000 error codes CENTRA-NF dengan test files dan dokumentasi.

## Setup (5 menit)

```bash
cd /workspaces/v1/tools
cargo build --release
```

Binary: `/workspaces/v1/tools/target/release/gen_errors`

## Uji Coba (Dry Run - Tidak menulis file)

```bash
cd /workspaces/v1
./tools/target/release/gen_errors \
  --layer 1 \
  --category syntax \
  --count 100 \
  --dry-run
```

Hasil: Lihat preview 100 errors tanpa membuat file.

## Generate 100 Error Pertama

```bash
cd /workspaces/v1

# 1. Buat folder
mkdir -p tests/ui/fail

# 2. Generate Layer 1 (Lexer) - 100 errors
./tools/target/release/gen_errors \
  --layer 1 \
  --category syntax \
  --count 100 \
  --test-dir /workspaces/v1/tests/ui/fail \
  --doc-file /workspaces/v1/docs/error-codes.md
```

**Hasil:**
- 100 file `.cnf` di `tests/ui/fail/` (l1001.cnf - l1100.cnf)
- 100 entri tabel di `docs/error-codes.md`
- Error codes: L1001 - L1100

## Verify Hasil

```bash
# Lihat satu test file
head -15 tests/ui/fail/l1001.cnf

# Lihat dokumentasi yang diupdate
tail -100 docs/error-codes.md | head -50

# Count files
ls tests/ui/fail/*.cnf | wc -l
```

## Generate lebih banyak untuk setiap layer

```bash
# Layer 2 (Parser) - 100 errors
./tools/target/release/gen_errors -l 2 -c syntax -n 100

# Layer 3 (IR) - 100 errors
./tools/target/release/gen_errors -l 3 -c semantic -n 100

# Layer 4 (Runtime) - 100 errors
./tools/target/release/gen_errors -l 4 -c runtime -n 100
```

## Script All Layer (625 errors each = 5000 total)

```bash
#!/bin/bash
cd /workspaces/v1
for layer in 1 2 3 4 5 6 7 8; do
  echo "Layer $layer..."
  ./tools/target/release/gen_errors \
    --layer $layer \
    --count 625 \
    --test-dir /workspaces/v1/tests/ui/fail \
    --doc-file /workspaces/v1/docs/error-codes.md
done
echo "✓ Generated 5000 errors!"
```

## Options

```
-l, --layer LAYER          Layer (1-8: Lexer/Parser/IR/Runtime/Security/Protocol/CLI/LSP)
-c, --category CATEGORY    Category: syntax | semantic | runtime
-n, --count COUNT          Jumlah errors (default 100)
-t, --test-dir PATH        Output test file folder (default: tests/ui/fail)
-d, --doc-file PATH        Documentation file (default: docs/error-codes.md)
    --dry-run              Preview tanpa menulis file
-h, --help                 Help
```

## Format Test File

```cnf
// Test file for error code: L1001
// Expected error: Invalid Missing character in instruction sequence -- expected valid UTF-8 encoding
// Category: Syntax
// Layer: 1 (Lexer)

IDENTIFICATION DIVISION.
    PROGRAM "test-invalid".
EVIRONMENT DIVISION.
    OS "invalid™utf8-chars".
```

## Format Docs Entry

| Code | Message | Example | Fix |
|------|---------|---------|-----|
| L1001 | Invalid Missing character in instruction sequence -- expected valid UTF-8 encoding | `cnf ... ` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |

## Detail docs

Lihat: `/workspaces/v1/tools/README.md`
