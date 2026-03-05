# CENTRA-NF Error Code Generator (gen_errors)

Skrip otomatis untuk menghasilkan error codes, test files, dan dokumentasi error secara masif.

## Fitur

- **Error Code Generation**: Membuat error codes dengan format `CNF-XYYY` (X = layer, YYY = nomor)
- **Test File Generator**: Membuat file `.cnf` di `tests/ui/fail/` dengan header `// expected-error: CNF-KODE`
- **Documentation Generator**: Secara otomatis menambah entri ke `docs/error-codes.md` dengan format tabel konsisten
- **Permutation Engine**: Menggunakan kombinasi kata kunci (Invalid, Missing, Overflow, etc.) dan tipe data (BINARY-BLOB, VIDEO-MP4, etc.) untuk variasi unik
- **Dry-Run Mode**: Preview output tanpa menulis file dengan flag `--dry-run`

## Struktur Folder

```
tools/
├── Cargo.toml          # Dependencies dan binary config
└── src/
    └── gen_errors.rs   # Main generator script
```

## Setup

### Build Binary

```bash
cd /workspaces/v1/tools
cargo build --release
```

Binary will be located at: `/workspaces/v1/tools/target/release/gen_errors`

## Usage

### Syntax

```bash
gen_errors [OPTIONS]
```

### Options

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--layer LAYER` | `-l` | `1` | Layer designation (1-8): Lexer, Parser, IR, Runtime, Security, Protocol, CLI, LSP |
| `--category CATEGORY` | `-c` | `syntax` | Error category: `syntax`, `semantic`, `runtime` |
| `--count COUNT` | `-n` | `100` | Number of errors to generate |
| `--test-dir PATH` | `-t` | `tests/ui/fail` | Output directory for test .cnf files |
| `--doc-file PATH` | `-d` | `docs/error-codes.md` | Documentation file to update |
| `--dry-run` | — | — | Preview output without writing files |

### Layer Mapping

| Layer | Designation | Name | Prefix |
|-------|-----------|------|--------|
| 1 | `-l 1` | Lexer | `L` |
| 2 | `-l 2` | Parser | `P` |
| 3 | `-l 3` | IR | `I` |
| 4 | `-l 4` | Runtime | `R` |
| 5 | `-l 5` | Security | `S` |
| 6 | `-l 6` | Protocol | `O` |
| 7 | `-l 7` | CLI | `C` |
| 8 | `-l 8` | LSP | `X` |

## Examples

### 1. Generate First 100 Lexer (Layer 1) Errors - Dry Run

```bash
cd /workspaces/v1
./tools/target/release/gen_errors \
  --layer 1 \
  --category syntax \
  --count 100 \
  --dry-run
```

**Expected Output:**
```
=== CENTRA-NF Error Code Generator ===
Layer: 1 (Lexer)
Category: Syntax
Target count: 100

Generated 100 error codes
Sample codes: L1001, L1002, L1003

=== DRY RUN: 100 errors would be generated ===

Code: L1001
Message: Invalid Missing character in instruction sequence -- expected valid UTF-8 encoding
...
```

### 2. Generate First 100 Lexer Errors - Actual

```bash
cd /workspaces/v1
mkdir -p tests/ui/fail
./tools/target/release/gen_errors \
  --layer 1 \
  --category syntax \
  --count 100 \
  --test-dir /workspaces/v1/tests/ui/fail \
  --doc-file /workspaces/v1/docs/error-codes.md
```

**Expected Output:**
```
✓ Created test file: tests/ui/fail/l1001.cnf
✓ Created test file: tests/ui/fail/l1002.cnf
...
✓ Updated documentation: docs/error-codes.md
✓ Success! Errors generated and documentation updated.
```

### 3. Generate 200 Parser (Layer 2) Errors

```bash
cd /workspaces/v1
./tools/target/release/gen_errors \
  --layer 2 \
  --category semantic \
  --count 200
```

### 4. Generate 500 Runtime (Layer 4) Errors with Relative Paths

```bash
cd /workspaces/v1
./tools/target/release/gen_errors \
  -l 4 \
  -c runtime \
  -n 500
```

### 5. Generate All 8 Layers with 625 Errors Each (5000 total)

```bash
cd /workspaces/v1
for layer in 1 2 3 4 5 6 7 8; do
  echo "Generating Layer $layer..."
  ./tools/target/release/gen_errors \
    --layer $layer \
    --category syntax \
    --count 625
done
```

## Generated Test File Format

**Example: `tests/ui/fail/l1001.cnf`**

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

## Generated Documentation Format

**Added to `docs/error-codes.md`:**

```markdown
## Layer 1: Lexer Errors

| Code | Message | Example | Fix |
|------|---------|---------|--------|
| L1001 | Invalid Missing character in instruction sequence -- expected valid UTF-8 encoding | ```cnf IDENTIFICATION DIVISION. ... ``` | Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping. |
| L1002 | Invalid Overflow character in variable declaration -- expected valid UTF-8 encoding | ... | ... |
```

## Error Message Generation Strategy

Generator menggunakan **kombinasi permutasi** dari:

1. **Keywords** (20 total):
   - Invalid, Missing, Overflow, Underflow, Unexpected, Illegal, Malformed, Unterminated, Undefined, Duplicate, Mismatch, Type, Constraint, Boundary, State, Order, Syntax, Semantic, Unmatched, Expected

2. **Data Types** (8 total):
   - BINARY-BLOB, VIDEO-MP4, IMAGE-JPG, FINANCIAL-DECIMAL, AUDIO-WAV, TEXT-UTF8, DOCUMENT-PDF, DATA-CSV

3. **Context** (8 supplements):
   - in division structure, in instruction sequence, in variable declaration, in expression, in control flow, in type annotation, in indentation, in encoding

Jumlah kombinasi potensial ≥ 28×8 = 224 kombinasi unik per layer, memungkinkan generasi hingga 5000 error codes dengan messaging yang variatif namun tetap bermakna.

## Performance Notes

- **100 errors**: ~100ms
- **1000 errors**: ~500ms  (saat membaca/menulis file)
- **5000 errors**: ~3-5s (bergantung disk I/O)

File descriptor limit: Default Linux = 1024, cukup untuk 5000 test files.

## Integration with Test Engine

Test files di `tests/ui/fail/*.cnf` dapat diintegrasikan dengan UI test runner:

```bash
# Pseudo test harness (belum diimplementasi)
for testfile in tests/ui/fail/*.cnf; do
  if ! cnf-compiler "$testfile" 2>&1 | grep -q "$(grep expected-error "$testfile")"; then
    echo "FAIL: $testfile"
  else
    echo "PASS: $testfile"
  fi
done
```

## Troubleshooting

### Error: "No such file or directory"

**Penyebab**: Path relative tidak dapat ditemukan dari working directory

**Solusi**: Gunakan absolute paths:

```bash
./tools/target/release/gen_errors \
  --test-dir /workspaces/v1/tests/ui/fail \
  --doc-file /workspaces/v1/docs/error-codes.md
```

### Error: "Short option '-c' is in use by both"

**Penyebab**: Versi lama skrip dengan konflik short options

**Solusi**: Rebuild dengan `cargo build --release`

### Dokumentasi tidak terupdate

**Penyebab**: File permissions atau path tidak valid

**Solusi**: 
1. Pastikan folder `docs/` ada
2. Gunakan absolute path untuk `--doc-file`
3. Check permissions: `ls -la docs/error-codes.md`

## Future Enhancements

- [ ] Dukungan custom keyword/type library via config file
- [ ] Format output selain Markdown (JSON, CSV, HTML)
- [ ] Incremental update (tidak overwrite section) dengan tracking
- [ ] Parallelized file writing untuk generasi > 1000 errors
- [ ] Integration dengan GitHub Actions untuk CI/CD

## License

Part of CENTRA-NF project. Subject to project governance in `.github/copilot-instructions.md`.
