# Migration to Unified Error System - Quick Start

Migrate from 5000 scattered `.cnf` files to one centralized `errors_master.yaml` file.

## Current State

- ❌ **Old**: 100+ test files in `tests/ui/fail/` 
- ❌ **Old**: Docs manually edited in `docs/error-codes.md`
- ❌ **Old**: Tests require filesystem I/O

## New State

- ✅ **New**: One `errors_master.yaml` with all test data
- ✅ **New**: Auto-generated docs from YAML
- ✅ **New**: In-memory testing (no file clutter)

## 3-Step Migration

### Step 1: Build Tools

```bash
cd /workspaces/v1/tools
cargo build
```

Binaries created:
- `target/debug/doc_gen`
- `target/debug/test_engine`
- `target/debug/gen_errors`

### Step 2: Verify YAML Has All Errors

```bash
cd /workspaces/v1

# Count entries in YAML
grep "code: \"" errors_master.yaml | wc -l

# Check metadata count matches
grep "current_count:" errors_master.yaml
```

**Important**: Before deleting test files, ensure YAML has all errors.

### Step 3: Remove Old Test Files (SAFE)

```bash
# All data is now in errors_master.yaml
rm -rf /workspaces/v1/tests/ui/fail/

# Verify deletion
ls /workspaces/v1/tests/ 

# No more ui/fail/ folder - this is expected!
```

## Verify System Works

### 1. Generate Docs

```bash
cd /workspaces/v1
./tools/target/debug/doc_gen \
  --input errors_master.yaml \
  --output docs/error-codes.md
```

**Result**: `docs/error-codes.md` regenerated from YAML ✓

### 2. Run Tests

```bash
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --verbose

# Expected: Each error code tested in-memory
# No external test files needed!
```

**Result**: Tests run from YAML memory, temp files auto-cleanup ✓

### 3. Add More Errors (Optional)

```bash
./tools/target/debug/gen_errors \
  --layer 1 \
  --category syntax \
  --count 50 \
  --yaml-file errors_master.yaml
```

**Result**: 50 new errors added to YAML ✓

## File Locations

**Before (Old System)**:
```
project/
├── tests/
│   └── ui/
│       └── fail/
│           ├── l1001.cnf
│           ├── l1002.cnf
│           ...
│           └── l1100.cnf  (100+ files)
└── docs/
    └── error-codes.md  (Hard to maintain)
```

**After (New System)**:
```
project/
├── errors_master.yaml  (Single source of truth!)
├── tests/  (ui/fail/ DELETED - not needed)
└── docs/
    └── error-codes.md  (Auto-generated from YAML)
```

## Workflow Changes

### Old Workflow
1. Edit individual `.cnf` files
2. Manually update `docs/error-codes.md`
3. Run tests against 100+ files
4. Version control 100+ file changes

### New Workflow
1. Edit `errors_master.yaml` (one file!)
2. Run `doc_gen` to auto-update docs
3. Run `test_engine` for in-memory tests
4. Version control one file change

## Command Reference

### Add Errors to YAML
```bash
./tools/target/debug/gen_errors \
  --layer 1 \
  --count 100 \
  --yaml-file errors_master.yaml
```

### Generate Documentation
```bash
./tools/target/debug/doc_gen \
  --input errors_master.yaml \
  --output docs/error-codes.md
```

### Run Tests (In-Memory)
```bash
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --verbose
```

### Test Single Layer
```bash
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --layer 1 \
  --verbose
```

### Test Verbose with Fail-Fast
```bash
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --fail-fast \
  --verbose
```

## Safety Checklist

Before deleting `tests/ui/fail/`:

- [ ] Backup `docs/error-codes.md`
- [ ] Verify `errors_master.yaml` has all errors
- [ ] Run `doc_gen` successfully
- [ ] Run `test_engine` without errors
- [ ] Verify generated docs look correct
- [ ] Commit `errors_master.yaml` to git

After safe verification:
- [ ] Delete `tests/ui/fail/` folder
- [ ] Delete individual `.cnf` test files
- [ ] Verify tests still work from YAML
- [ ] Commit cleanup to git

## Benefits Summary

| Item | Old | New |
|------|-----|-----|
| Test Files | 5000+ scattered | 1 YAML file |
| Disk Space | ~1.4 MB | ~50 KB |
| Git Commits | 5000 file diffs | 1 file diff |
| Test Speed | Filesystem I/O | In-memory |
| Consistency | Manual sync | Auto-sync |
| Maintenance | Hard (5000 files) | Easy (1 file) |
| Search | grep across files | grep in 1 file |
| Documentation | Manual updates | Auto-generated |

## Troubleshooting

### "Cannot find errors_master.yaml"
```bash
# Ensure you're in project root
cd /workspaces/v1
ls errors_master.yaml
```

### "Tests fail with compiler not found"
```bash
# Provide path to cnf-compiler
./tools/target/debug/test_engine \
  --yaml-file errors_master.yaml \
  --compiler /path/to/cnf-compiler
```

### "Generated docs look empty"
```bash
# Check YAML structure is correct
grep "^  - code:" errors_master.yaml | wc -l

# Regenerate
./tools/target/debug/doc_gen --verbose
```

## Next Steps

1. ✅ Run Step 1: Build Tools
2. ✅ Run Step 2: Verify YAML
3. ✅ Run Step 3: Delete old test files
4. ✅ Verify system works
5. Start adding new errors to YAML only!

---

**Questions?** See: `/workspaces/v1/UNIFIED_ERROR_SYSTEM.md`
