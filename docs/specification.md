# CENTRA-NF Language Specification

**Version 1.0.0 - Stable Release with Governance**

**Auto-generated from errors_registry.json**

## Error Coverage

Total error codes: **2000**

### Errors by Layer

- **Layer 1: Lexer** (400 codes)
- **Layer 2: Parser** (400 codes)
- **Layer 3: IR** (400 codes)
- **Layer 4: Runtime** (400 codes)
- **Layer 5: Security** (400 codes)

---

## 1. Introduction

CENTRA-NF is a domain-specific programming language designed for secure data processing and compression operations. It follows a COBOL-inspired structure with four mandatory divisions and optional governance features for compliance and access control.

## 2. Language Structure

Every CENTRA-NF program must contain exactly four divisions in this strict order:

```cobol
IDENTIFICATION DIVISION.
ENVIRONMENT DIVISION.
DATA DIVISION.
PROCEDURE DIVISION.
```

An optional `GOVERNANCE DIVISION` may be inserted between `DATA DIVISION` and `PROCEDURE DIVISION` for compliance and access control:

```cobol
IDENTIFICATION DIVISION.
ENVIRONMENT DIVISION.
DATA DIVISION.
GOVERNANCE DIVISION.  // Optional
PROCEDURE DIVISION.
```

### 2.1 IDENTIFICATION DIVISION

Declares the program identity:

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. "MyProgram".
    AUTHOR. "Developer Name".
    DATE-WRITTEN. "2026-03-05".
```

### 2.2 ENVIRONMENT DIVISION

Configures execution environment with quoted string values:

```cobol
ENVIRONMENT DIVISION.
    OS "Linux".
    ARCH "x86_64".
    VERSION "0.2.0".
```

### 2.3 DATA DIVISION

Declares variables with explicit data types.  An optional `AS <identifier>` clause
may be used to give the variable a custom name; if omitted the type name itself is
used as the variable name.

```cobol
DATA DIVISION.
    INPUT-VIDEO AS VIDEO-MP4.
    OUTPUT-IMAGE AS IMAGE-JPG.
    FINANCIAL-DATA AS FINANCIAL-DECIMAL.
    RESULT AS BINARY-BLOB.
```

**Supported Data Types:**
- `VIDEO-MP4`: MP4 video files
- `IMAGE-JPG`: JPEG image files
- `AUDIO-WAV`: WAV audio files
- `CSV-TABLE`: CSV data tables
- `JSON-OBJECT`: JSON structured data
- `XML-DOCUMENT`: XML documents
- `PARQUET-TABLE`: Parquet columnar data
- `BINARY-BLOB`: Generic binary data
- `FINANCIAL-DECIMAL`: High-precision decimal numbers

### 2.4 GOVERNANCE DIVISION (Optional)

Defines governance policies, regulations, and access controls for the program:

```cobol
GOVERNANCE DIVISION.
    POLICY COMPLIANCE-POLICY FORMULA "G F (compress -> verify)".
    REGULATION GDPR CLAUSE "Article 5 - Data minimization".
    ACCESS-CONTROL USER "default" RESOURCE "data" ACTION "COMPRESS".
    AUDIT-LEDGER "Program execution started".
```

**Governance Statements:**
- `POLICY name FORMULA "ltl_formula"`: Define LTL policy formulas
- `REGULATION standard CLAUSE "description"`: Reference regulatory requirements
- `ACCESS-CONTROL USER "user" RESOURCE "resource" ACTION "action"`: Define access permissions
- `AUDIT-LEDGER "message"`: Log audit messages
- `DECISION-QUORUM VOTES "count" THRESHOLD "min"`: Set decision thresholds

### 2.5 PROCEDURE DIVISION

Contains executable statements and operations:

```cobol
PROCEDURE DIVISION.
    COMPRESS INPUT-VIDEO AS OUTPUT-IMAGE.
    VERIFY-INTEGRITY OUTPUT-IMAGE.
    DISPLAY "Processing complete".
```

## 3. Operations

CENTRA-NF supports 12 core operations across compression, encryption, formatting, and data processing:

### 3.1 Compression Operations
- `COMPRESS source AS target`: Compress data using L1-L3 protocol
- `VERIFY-INTEGRITY target`: Verify data integrity with SHA-256

### 3.2 Encryption Operations
- `ENCRYPT source`: Encrypt data with AES-256
- `DECRYPT source`: Decrypt AES-256 encrypted data

### 3.3 Formatting Operations
- `TRANSCODE source AS target`: Convert between formats
- `CONVERT source AS target`: Type conversion
- `FILTER source AS target`: Data filtering

### 3.4 Data Processing Operations
- `AGGREGATE source AS target`: Data aggregation
- `MERGE source AS target`: Data merging
- `SPLIT source AS target`: Data splitting
- `VALIDATE source`: Data validation
- `EXTRACT source AS target`: Data extraction

## 4. Control Flow (Planned for v0.3.0)

```cobol
PROCEDURE DIVISION.
    IF CONDITION THEN
        COMPRESS INPUT-VIDEO AS OUTPUT.
    ELSE
        DISPLAY "Invalid input".
    END-IF.

    PERFORM UNTIL END-OF-DATA
        PROCESS-RECORD.
    END-PERFORM.
```

## 5. Functions and Procedures (Planned for v0.3.0)

```cobol
PROCEDURE DIVISION.
    CALL PROCESS-VIDEO USING INPUT-VIDEO OUTPUT-IMAGE.

PROCESS-VIDEO SECTION.
    COMPRESS INPUT-VIDEO AS OUTPUT-IMAGE.
    VERIFY-INTEGRITY OUTPUT-IMAGE.
EXIT SECTION.
```

## 6. Error Handling

CENTRA-NF implements comprehensive error handling with 2000+ error codes:

- **Lexer Errors (L1001-L1400)**: Token recognition and syntax validation
- **Parser Errors (L2001-L2400)**: Division order and structure validation
- **IR Errors (L3001-L3400)**: Type checking and lowering validation
- **Runtime Errors (L4001-L4400)**: Execution and operation validation
- **Security Errors (L5001-L5400)**: Cryptographic operation validation

## 7. Examples

### 7.1 Basic Compression

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. "VideoCompressor".

ENVIRONMENT DIVISION.
    OS "Linux".
    ARCH "x86_64".

DATA DIVISION.
    INPUT-FILE AS VIDEO-MP4.
    OUTPUT-FILE AS BINARY-BLOB.

PROCEDURE DIVISION.
    COMPRESS INPUT-FILE AS OUTPUT-FILE.
    VERIFY-INTEGRITY OUTPUT-FILE.
    DISPLAY "Compression complete".
```

### 7.2 Data Processing Pipeline

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. "DataPipeline".

ENVIRONMENT DIVISION.
    OS "Linux".

DATA DIVISION.
    RAW-DATA AS CSV-TABLE.
    PROCESSED-DATA AS JSON-OBJECT.
    ENCRYPTED-DATA AS BINARY-BLOB.

PROCEDURE DIVISION.
    VALIDATE RAW-DATA.
    CONVERT RAW-DATA AS PROCESSED-DATA.
    ENCRYPT PROCESSED-DATA AS ENCRYPTED-DATA.
    VERIFY-INTEGRITY ENCRYPTED-DATA.
```

## 8. Implementation Status

### Completed Features (v0.2.0)
- ✅ Four-division structure with strict ordering
- ✅ 9 data types with type validation
- ✅ 12 core operations (compression, encryption, processing)
- ✅ SHA-256 integrity verification
- ✅ AES-256 encryption/decryption
- ✅ Comprehensive error system (2000+ codes)
- ✅ JSON-based error registry
- ✅ Auto-generated documentation

### Planned Features (v0.3.0)
- 🔄 Control flow (IF/ELSE, loops)
- 🔄 User-defined functions and procedures
- 🔄 Advanced type system with generics
- 🔄 Module system and imports
- 🔄 Standard library expansion
- 🔄 Performance optimizations

### Future Features (v0.4.0+)
- 🔄 Concurrency and parallelism
- 🔄 External API integrations
- 🔄 Advanced cryptographic primitives
- 🔄 Machine learning operations
- 🔄 Real-time processing capabilities

## 9. Architecture

CENTRA-NF follows a layered architecture:

```
User Code (.cnf)
    ↓
Lexer → Parser → AST → IR → Runtime → Protocol/Security
    ↑           ↑       ↑       ↑           ↑
  Tokenize   Syntax   Tree   Lower    Execute    Compress/Encrypt
```

### Layer Responsibilities
- **Compiler**: Source code to deterministic IR
- **Runtime**: IR execution with 8-layer DAG scheduler
- **Security**: SHA-256 hashing and AES-256 encryption
- **Protocol**: L1-L3 compression (CORE-FROZEN)

## 10. Quality Gates

Every commit must pass:
- ✅ `cargo check --all`
- ✅ `cargo test --all --lib`
- ✅ `cargo test --all --test '*'` (integration)
- ✅ `cargo fmt --all -- --check`
- ✅ `cargo clippy --all -- -D warnings`
- ✅ `cargo build --all --release`
- ✅ Layer boundary verification
- ✅ CORE-FROZEN integrity check

## 11. Development Roadmap

### Phase 1: Core Language (v0.2.0) ✅
- Basic syntax and operations
- Error management system
- Runtime execution engine

### Phase 2: Advanced Features (v0.3.0) 🔄
- Control flow and functions
- Type system enhancements
- Standard library

### Phase 3: Enterprise Features (v0.4.0) 📋
- Concurrency and performance
- External integrations
- Advanced security

### Phase 4: Ecosystem (v0.5.0) 📋
- Package management
- Tooling ecosystem
- Community adoption

---

**Last Updated:** 2026-03-05
**Version:** 0.2.0
**Status:** Solid foundation with comprehensive error handling
