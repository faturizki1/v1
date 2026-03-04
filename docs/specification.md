# CENTRA-NF Language Specification

## Version 0.1.0

---

## Overview

CENTRA-NF is a deterministic, fail-fast domain-specific language (DSL) for long-running systems in data centers. Programs are deterministic: the same source code always produces the same execution behavior.

---

## Division Structure

Every `.cnf` program consists of exactly **four divisions** in this fixed order:

1. `IDENTIFICATION DIVISION`
2. `ENVIRONMENT DIVISION`
3. `DATA DIVISION`
4. `PROCEDURE DIVISION`

Deviating from this order is a hard parse error.

### IDENTIFICATION DIVISION

Declares program metadata.

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. VideoProcessor.
    AUTHOR. Infrastructure-Team.
    VERSION. 1-0-0.
```

Fields:
- `PROGRAM-ID` (required): Program identifier
- `AUTHOR` (optional): Author name
- `VERSION` (optional): Version string

### ENVIRONMENT DIVISION

Configures runtime environment. All values **must be quoted strings**.

```cobol
ENVIRONMENT DIVISION.
    OS "Linux".
    ARCH "x86_64".
    RUNTIME-VERSION "1.0".
```

Unquoted values are syntax errors.

### DATA DIVISION

Declares variables with data types.

```cobol
DATA DIVISION.
    INPUT BUFFER VIDEO-MP4.
    OUTPUT PREVIEW IMAGE-JPG.
    AUDIT-LOG FINANCIAL-DECIMAL.
```

#### Supported Data Types

| Type | Description |
|------|-------------|
| `VIDEO-MP4` | High-throughput video buffer |
| `IMAGE-JPG` | Image buffer (zero-copy target) |
| `FINANCIAL-DECIMAL` | Deterministic decimal for financial workloads |

### PROCEDURE DIVISION

Specifies execution sequence.

```cobol
PROCEDURE DIVISION.
    COMPRESS BUFFER.
    VERIFY-INTEGRITY BUFFER.
```

#### Supported Operations

| Operation | Effect |
|-----------|--------|
| `COMPRESS <target>` | Compress buffer via L1-L2-L3 |
| `VERIFY-INTEGRITY <target>` | Compute SHA-256 digest |

---

## Compiler Pipeline

```
Source (.cnf)
    ↓
[Lexer] — tokenize, reject unknown chars
    ↓
[Parser] — enforce division order, build AST
    ↓
[AST] — explicit, minimal tree
    ↓
[IR] — deterministic lowering
    ↓
[Runtime] — dispatch to crates
```

### Error Philosophy

**Fail fast**: Invalid input generates explicit, loud errors. No silent fallback.

| Stage | Error Example |
|-------|---|
| Lexer | "Unrecognized character '@' at line 5:8" |
| Parser | "Expected ENVIRONMENT DIVISION, got DATA DIVISION" |
| Parser | "Expected quoted string in ENVIRONMENT, got unquoted Linux" |
| Runtime | "Buffer 'missing_var' not declared in DATA DIVISION" |

---

## Runtime Execution Model

### DAG Scheduling

Execution uses an 8-layer Directed Acyclic Graph (DAG).
Scheduler executes each layer **completely** before moving to the next.

Layer execution is deterministic and sequential.

### Memory Model

- All buffers use `Vec<u8>` ownership
- Move semantics preferred over cloning
- No global mutable state
- Option<T> or Result<T, E> for all partial operations

### Zero-Copy

`VIDEO-MP4` and `IMAGE-JPG` buffers use move semantics to avoid copies.

---

## Security Model

### VERIFY-INTEGRITY

Computes SHA-256 digest of target buffer.

```cobol
VERIFY-INTEGRITY BUFFER.
```

Returns 64-character hex-encoded digest.

### Isolation

All cryptographic operations exist **only** in `cnf-security`.
No other crate performs hashing.

---

## Complete Example

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. ArchivePipeline.
    AUTHOR. DataCenter-Team.
    VERSION. 1-0.

ENVIRONMENT DIVISION.
    OS "Linux".
    ARCH "x86_64".
    RUNTIME-VERSION "1.0".

DATA DIVISION.
    INPUT RAW-FEED VIDEO-MP4.
    OUTPUT PREVIEW IMAGE-JPG.
    LEDGER AUDIT-LOG FINANCIAL-DECIMAL.

PROCEDURE DIVISION.
    COMPRESS RAW-FEED.
    VERIFY-INTEGRITY RAW-FEED.
    COMPRESS PREVIEW.
    VERIFY-INTEGRITY AUDIT-LOG.
```

---

## Determinism Guarantee

Same program source → same execution behavior always.

No randomness, timers, or environment-dependent behavior is allowed in the runtime path.

---

## Backward Compatibility

The `cobol-protocol-v153` crate is CORE-FROZEN.
No modifications to this crate will be made to ensure protocol stability.

---

## Future Operations

Planned expansions (not yet implemented):

- `TRANSCODE` — convert media formats
- `FILTER` — process structured data
- `AGGREGATE` — reduce buffers

New data types planned:

- `AUDIO-WAV`
- `CSV-TABLE`
- `BINARY-BLOB`

---

## Contact

For specification questions or issues, contact the infrastructure team.
