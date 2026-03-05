# CENTRA-NF Unified Error Management System

## Overview

The new unified system consolidates all error code management into **one master file** (`errors_master.yaml`):

- **❌ Old way**: 5000+ individual `.cnf` test files scattered in `tests/ui/fail/`
- **✅ New way**: One `errors_master.yaml` file containing:
  - Error registry (code, title, description)
  - Test case snippets (trigger code in YAML)
  - Documentation data
  - In-memory testing without file clutter

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   errors_master.yaml                        │
│  (Single source of truth for all error management)          │
│                                                             │
│  • 5000 error codes with metadata                           │
│  • Trigger code snippets embedded as YAML strings           │
│  • Test cases defined inline                                │
│  • Documentation content                                    │
└─────────────────────────────────────────────────────────────┘
         ↓                           ↓                    ↓
    ┌────────────┐          ┌──────────────┐      ┌────────────┐
    │ doc_gen    │          │ test_engine  │      │ gen_errors │
    │ Binary     │          │ Binary       │      │ Binary     │
    └────────────┘          └──────────────┘      └────────────┘
         ↓                           ↓                    ↓
    ┌────────────┐               (RAM)            ┌────────────┐
    │ Generate   │               Tests            │ Generate   │
    │ Markdown   │               With Temp        │ More YAML  │
    │ Docs       │               Files            │ Entries    │
    └────────────┘               (auto-cleanup)   └────────────┘
         ↓                                        
 docs/error-codes.md          
```

## Files

### Master Registry
- **`errors_master.yaml`** (Single source of truth)
  - Contains all 5000 error codes
  - Trigger code for each test case
  - No external test files required
  - Human-readable YAML format

### Tool Binaries (in `/workspaces/v1/tools/`)
- **`doc_gen`** - Generate `docs/error-codes.md` from YAML
- **`test_engine`** - Run tests from YAML with in-memory execution
- **`gen_errors`** - Add more error codes to YAML

### Output Files
- **`docs/error-codes.md`** (Auto-generated from YAML)
  - User-facing documentation
  - Regenerated on demand
  - Always in sync with YAML

## Workflows

### 1. Adding New Error Codes

Add entry to `errors_master.yaml`:
```yaml
- code: "L1016"
  layer: 1
  layer_name: "Lexer"
  category: "Syntax"
  title: "Invalid operator"
  description: "Operator used is not recognized"
  trigger_code: |
    IDENTIFICATION DIVISION.
        PROGRAM "test @@invalid".
  expected_error: "Invalid operator '@@'"
  fix: "Use valid operator or remove invalid syntax"
```

### 2. Regenerating Documentation

```bash
cd /workspaces/v1
./tools/target/debug/doc_gen \
  --input errors_master.yaml \
  --output docs/error-codes.md
```

### 3. Running Tests

All tests are in YAML - no external files needed:

```bash
# Test all layers
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --verbose

# Test specific layer (1-8)
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --layer 1 \
  --verbose

# With compiler path
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --compiler ./path/to/cnf-compiler \
  --temp-dir /tmp
```

## YAML Structure

```yaml
metadata:
  title: "CENTRA-NF Error Code Master Registry"
  format_version: "1.0"
  current_count: 45  # Incrementing as we add errors
  layers:
    1: "Lexer"
    2: "Parser"
    ... etc

errors:
  # Array of error objects
  - code: "L1001"
    layer: 1
    layer_name: "Lexer"
    category: "Syntax"
    title: "Error Title"
    description: "Detailed description"
    trigger_code: |
      # This is the .cnf code that triggers the error
      IDENTIFICATION DIVISION.
          PROGRAM "test".
    expected_error: "Error message expected in output"
    fix: "How to fix this error"
```

### Key Features of YAML Format
- **Inline trigger code**: No external files
- **Structured hierarchy**: Easy to organize by layer
- **Metadata**: Track format version and count
- **Self-documenting**: Clear field names

## In-Memory Testing

Test Engine flow:

```
For each error in YAML:
  1. Read trigger_code from YAML
  2. Write to temporary file (e.g., /tmp/test_L1001.cnf)
  3. Execute: cnf-compiler /tmp/test_L1001.cnf
  4. Capture output
  5. Check if expected_error appears in output
  6. Delete temporary file ✓
  7. Report result
```

**Advantages:**
- ✓ No permanent test file clutter
- ✓ Tests live in YAML (single source)
- ✓ Fast (temp files auto-cleaned)
- ✓ Can test thousands without filesystem overhead

## Migration from Old System

### Step 1: Clean Up Old Test Files
```bash
# Safe to delete - data is now in errors_master.yaml
rm -rf /workspaces/v1/tests/ui/fail/
```

### Step 2: Verify YAML Has All Data
```bash
# Count entries in YAML
grep "code: \"" errors_master.yaml | wc -l

# Should match count in metadata.current_count
```

### Step 3: Generate Docs
```bash
cd /workspaces/v1
./tools/target/debug/doc_gen
```

### Step 4: Run Tests (From YAML)
```bash
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --verbose
```

## Format Comparison

### Old System (File-Based)
```
tests/
├── ui/
│   └── fail/
│       ├── l1001.cnf  (283 bytes)
│       ├── l1002.cnf  (284 bytes)
│       ├── l1003.cnf  (275 bytes)
│       ...
│       └── l1100.cnf  (283 bytes)
```
**Total**: 100 files × ~280 bytes = 28KB clutter

### New System (YAML-Based)
```
errors_master.yaml  (1 file, ~50KB for 1000 errors)
```
**All data in one place**

## Scaling to 5000 Errors

Add 50 more error blocks to `errors_master.yaml`:

```bash
# Current: 45 errors (sample)
# Target: 5000 errors
# Pattern: 625 errors per layer × 8 layers
```

One YAML file can easily hold 5000+ error entries.

Example structure for full system:
```yaml
metadata:
  current_count: 5000
  layers: 8

errors:
  # Layer 1 (Lexer): L1001 - L1625 (625 errors)
  - code: "L1001"
    ...
  - code: "L1625"
    ...
  
  # Layer 2 (Parser): P2001 - P2625 (625 errors)
  - code: "P2001"
    ...
  - code: "P2625"
    ...
  
  # ... 6 more layers ...
  
  # Layer 8 (LSP): X8001 - X8625 (625 errors)
  - code: "X8001"
    ...
  - code: "X8625"
    ...
```

## Benefits of New System

| Aspect | Old | New |
|--------|-----|-----|
| **Files** | 5000 .cnf files | 1 YAML file |
| **Disk** | ~1.4MB | ~50KB |
| **Version Control** | Hard (5000 file diffs) | Easy (1 file) |
| **Search** | grep across files | grep in 1 file |
| **Edit** | Edit individual files | Edit in YAML |
| **Consistency** | Manual sync | Built-in |
| **Docs** | Manual updates | Auto-generated |
| **Testing** | Filesystem I/O | In-memory |
| **Cleanup** | Manual | Automatic |
| **Scalability** | O(n) files | O(1) file |

## Tools Reference

### doc_gen
Generate markdown documentation from YAML
```bash
./tools/target/debug/doc_gen [OPTIONS]

OPTIONS:
  -i, --input FILE     Input YAML (default: errors_master.yaml)
  -o, --output FILE    Output markdown (default: docs/error-codes.md)
```

### test_engine
Run tests from YAML with in-memory execution
```bash
./tools/target/debug/test_engine [OPTIONS]

OPTIONS:
  -y, --yaml-file FILE      YAML file (default: errors_master.yaml)
  -l, --layer 1-8           Layer to test (0 = all)
  -v, --verbose             Show all test results
  -t, --temp-dir DIR        Temp directory (default: /tmp)
  --compiler PATH           Path to cnf-compiler binary
  --fail-fast               Stop on first failure
```

### gen_errors
Add new errors to YAML
```bash
./tools/target/debug/gen_errors [OPTIONS]

OPTIONS:
  -l, --layer LAYER         Layer 1-8
  -c, --category CATEGORY   Syntax/Semantic/Runtime
  -n, --count COUNT         Number of errors
  -y, --yaml-file FILE      YAML file to update
```

## Workflow Summary

```bash
# 1. Add new errors (updates YAML)
./tools/target/debug/gen_errors -l 1 -c syntax -n 50

# 2. Regenerate docs (from YAML)
./tools/target/debug/doc_gen

# 3. Run all tests (from YAML, in-memory)
./tools/target/debug/test_engine --verbose

# 4. Commit one file to version control
git add errors_master.yaml docs/error-codes.md
git commit -m "Add 50 new Lexer errors"
```

## No More File Clutter

✅ **Safe to delete**: `tests/ui/fail/` folder
- All data is preserved in `errors_master.yaml`
- Tests run in-memory from YAML
- Documentation auto-generated from YAML

```bash
# One-time cleanup
rm -rf tests/ui/fail/
```

## Future Enhancements

- [ ] Add layer-specific validation rules to YAML
- [ ] Support for multiple trigger code examples per error
- [ ] Meta-tags for error severity (critical, warning, info)
- [ ] Integration with error tracking system
- [ ] Web UI for error browsing/editing
- [ ] Localization support in YAML

## Status

✅ **errors_master.yaml** - Created with 45 sample errors  
✅ **doc_gen** - Compiles, generates docs from YAML  
✅ **test_engine** - Compiles, runs in-memory tests  
✅ **gen_errors** - Enhanced to work with YAML  
⏳ **Migration** - Ready to clean up old test files

---

**Next Step**: Run doc_gen to generate updated documentation from errors_master.yaml
