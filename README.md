# CENTRA-NF

> **Domain-Specific Language for data center optimization, high-performance compression, multimedia processing, and long-running systems.**

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](#prerequisites)
[![License](https://img.shields.io/badge/license-read--only-blue)](#license)
[![Version](https://img.shields.io/badge/version-1.0.0-informational)](#)

---

## Daftar Isi

- [Gambaran Umum](#gambaran-umum)
- [Arsitektur Workspace](#arsitektur-workspace)
- [Prerequisites](#prerequisites)
- [Instalasi & Setup](#instalasi--setup)
- [Menjalankan Build & Test](#menjalankan-build--test)
- [Struktur Bahasa (.cnf)](#struktur-bahasa-cnf)
- [Compiler Pipeline](#compiler-pipeline)
- [Runtime Engine](#runtime-engine)
- [Security Model](#security-model)
- [Panduan Kontribusi](#panduan-kontribusi)
- [Menambah Operasi Baru](#menambah-operasi-baru)
- [Menambah Data Type Baru](#menambah-data-type-baru)
- [Konvensi Kode](#konvensi-kode)
- [Troubleshooting](#troubleshooting)
- [Roadmap](#roadmap)

---

## Gambaran Umum

CENTRA-NF adalah DSL dengan sintaks terinspirasi COBOL yang dikompilasi ke runtime Rust. Setiap program `.cnf` melewati pipeline deterministik:

```
Lexer → Parser → AST → IR → Runtime Call
```

Tiga prinsip desain yang tidak boleh dilanggar:

1. **Fail fast** — setiap input tidak valid menghasilkan error yang jelas; tidak ada silent failure
2. **Zero global state** — tidak ada `static mut`; thread safety dijamin secara struktural
3. **Satu tanggung jawab per modul** — setiap crate memiliki boundary yang tegas

---

## Arsitektur Workspace

```
centra-nf/
├── crates/
│   ├── cnf-compiler/          # Frontend: Lexer, Parser, AST, IR
│   │   ├── src/
│   │   │   ├── lexer.rs       # Tokenisasi source .cnf
│   │   │   ├── parser.rs      # Parsing division & enforcement urutan
│   │   │   ├── ast.rs         # Representasi AST minimal & eksplisit
│   │   │   └── ir.rs          # Lowering AST → IR deterministik
│   │   └── tests/
│   ├── cnf-runtime/           # Engine: buffer, DAG, scheduler
│   │   ├── src/
│   │   │   ├── dag.rs         # 8-layer Directed Acyclic Graph
│   │   │   ├── scheduler.rs   # Eksekusi layer-by-layer deterministik
│   │   │   └── runtime.rs     # Dispatch IR → operasi konkret
│   │   └── tests/
│   ├── cnf-security/          # SHA-256 integrity audit
│   │   ├── src/
│   │   │   └── lib.rs         # sha256_hex() — satu-satunya lokasi crypto
│   │   └── tests/
│   └── cobol-protocol-v153/   # ⛔ CORE-FROZEN — jangan modifikasi
│       └── src/
│           └── lib.rs         # compress_l1_l3() — protokol L1, L2, L3
├── docs/
│   └── specification.md       # Spesifikasi bahasa resmi
├── examples/
│   └── simple.cnf             # Program CNF minimal
├── Cargo.toml                 # Workspace root
└── README.md
```

### Batas Tanggung Jawab Antar Crate

| Crate | Boleh dilakukan | Tidak boleh |
|---|---|---|
| `cnf-compiler` | Tokenisasi, parsing, AST, IR | Mengeksekusi operasi, mengakses buffer runtime |
| `cnf-runtime` | Dispatch IR, kelola buffer, jalankan DAG | Parsing source, operasi crypto |
| `cnf-security` | SHA-256 hashing | Parsing, runtime dispatch, kompresi |
| `cobol-protocol-v153` | Kompresi L1–L3 | **Apapun selain itu — CORE-FROZEN** |

> ⛔ **`cobol-protocol-v153` adalah CORE-FROZEN.** Jangan modifikasi crate ini dalam kondisi apapun. Protocol stability dan backward compatibility bergantung pada crate ini tetap tidak berubah. Semua logic kompresi harus tetap terisolasi di dalamnya.

---

## Prerequisites

| Dependency | Versi Minimum | Catatan |
|---|---|---|
| Rust toolchain | 1.75+ | Install via `rustup` |
| Cargo | 1.75+ | Bundled bersama Rust |
| OS | Linux, macOS, Windows | Diuji di Linux x86_64 |

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup update stable
```

Verifikasi instalasi:

```bash
rustc --version   # rustc 1.75.0 (atau lebih baru)
cargo --version   # cargo 1.75.0 (atau lebih baru)
```

> ⚠️ **Catatan dev container:** Rust toolchain mungkin belum terinstal di container development. Jalankan perintah di atas sebelum melanjutkan. Tanpa toolchain, `cargo check` dan `cargo test` tidak dapat berjalan.

---

## Instalasi & Setup

```bash
# Clone repository
git clone <repo-url> centra-nf
cd centra-nf

# Verifikasi semua crate dapat dikompilasi
cargo check --all

# Jalankan seluruh test suite
cargo test --all
```

Jika `cargo check --all` berhasil tanpa error, environment siap untuk pengembangan.

---

## Menjalankan Build & Test

### Build

```bash
# Check semua crate (cepat, tanpa menghasilkan binary)
cargo check --all

# Build semua crate dalam mode debug
cargo build --all

# Build dalam mode release (untuk benchmarking)
cargo build --all --release
```

### Test

```bash
# Jalankan semua test di seluruh workspace
cargo test --all

# Test satu crate spesifik
cargo test -p cnf-compiler
cargo test -p cnf-runtime
cargo test -p cnf-security

# Jalankan satu test function spesifik
cargo test -p cnf-compiler test_lexer_valid_keywords

# Tampilkan output dari test yang lulus (bukan hanya yang gagal)
cargo test --all -- --nocapture

# Jalankan test secara berurutan (berguna untuk debug)
cargo test --all -- --test-threads=1
```

### Linting & Formatting

```bash
# Format kode (wajib sebelum commit)
cargo fmt --all

# Jalankan linter
cargo clippy --all -- -D warnings

# Cek tanpa mengubah file (untuk CI)
cargo fmt --all -- --check
```

---

## Struktur Bahasa (.cnf)

Setiap file `.cnf` terdiri dari tepat empat division, dideklarasikan **dalam urutan tetap**. Urutan yang salah adalah hard parse error.

### Urutan Division

```
IDENTIFICATION DIVISION.
ENVIRONMENT DIVISION.
DATA DIVISION.
PROCEDURE DIVISION.
```

### Semantik Per Division

#### `IDENTIFICATION DIVISION`
Metadata program. Free-form identifiers, diakhiri titik.

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. VideoProcessor.
    AUTHOR. DataCenter-Team.
    VERSION. 1-0.
```

#### `ENVIRONMENT DIVISION`
Pasangan key/value untuk konfigurasi runtime. Nilai **harus** berupa quoted string.

```cobol
ENVIRONMENT DIVISION.
    OS "Linux".
    ARCH "x86_64".
    RUNTIME-VERSION "1.0".
```

> Nilai tanpa tanda kutip adalah syntax error — parser akan gagal dengan pesan eksplisit.

#### `DATA DIVISION`
Deklarasi variabel dengan tipe data. Format: `<IDENTIFIER> <DATA-TYPE>.`

```cobol
DATA DIVISION.
    INPUT VIDEO-MP4.
    THUMBNAIL IMAGE-JPG.
    LEDGER FINANCIAL-DECIMAL.
```

**Data types yang didukung:**

| Type | Deskripsi |
|---|---|
| `VIDEO-MP4` | Buffer video high-throughput |
| `IMAGE-JPG` | Buffer image (target zero-copy) |
| `FINANCIAL-DECIMAL` | Desimal deterministik untuk workload finansial |

#### `PROCEDURE DIVISION`
Urutan operasi yang akan dieksekusi. Argumen diterima parser tapi divalidasi saat runtime.

```cobol
PROCEDURE DIVISION.
    COMPRESS INPUT.
    VERIFY-INTEGRITY INPUT.
```

**Operasi yang didukung:**

| Operasi | Crate yang dipanggil | Efek |
|---|---|---|
| `COMPRESS` | `cobol-protocol-v153` | Kompresi buffer via L1→L2→L3 |
| `VERIFY-INTEGRITY` | `cnf-security` | Hitung SHA-256 dan return hex digest |

### Contoh Program Lengkap

```cobol
IDENTIFICATION DIVISION.
    PROGRAM-ID. ArchivePipeline.
    AUTHOR. Infra-Team.

ENVIRONMENT DIVISION.
    OS "Linux".
    ARCH "x86_64".

DATA DIVISION.
    RAW-FEED VIDEO-MP4.
    PREVIEW IMAGE-JPG.
    AUDIT-LOG FINANCIAL-DECIMAL.

PROCEDURE DIVISION.
    COMPRESS RAW-FEED.
    VERIFY-INTEGRITY RAW-FEED.
    COMPRESS PREVIEW.
    VERIFY-INTEGRITY AUDIT-LOG.
```

---

## Compiler Pipeline

### Tahapan

```
Source (.cnf)
     │
     ▼
  [Lexer]          — tokenisasi; gagal pada karakter tidak dikenal
     │
     ▼
  [Parser]         — enforce urutan division; gagal jika salah urutan
     │
     ▼
  [AST]            — tree minimal & eksplisit; tidak ada node implisit
     │
     ▼
  [IR]             — lowering deterministik; input sama → IR sama selalu
     │
     ▼
  [Runtime Call]   — dispatch ke cnf-runtime, cnf-security, cobol-protocol-v153
```

### Error Philosophy

> **"Errors must be loud and fail on invalid syntax."**

Setiap stage menghasilkan error yang eksplisit dan menghentikan kompilasi:

| Kondisi Error | Stage | Perilaku |
|---|---|---|
| Karakter tidak dikenal | Lexer | Halt + pesan dengan posisi baris:kolom |
| Division salah urutan | Parser | Halt + sebutkan division yang diharapkan |
| Nilai ENVIRONMENT tanpa kutip | Parser | Halt + tunjukkan token yang diterima |
| Keyword PROCEDURE tidak dikenal | Parser | Halt + daftar keyword valid |
| Dispatch runtime gagal | Runtime | Panic + exit non-zero |

---

## Runtime Engine

### DAG Execution

Runtime mengeksekusi IR melalui DAG (Directed Acyclic Graph) 8-layer di `cnf-runtime/src/dag.rs`. Scheduler (`scheduler.rs`) menjalankan setiap layer secara berurutan — satu layer harus selesai penuh sebelum layer berikutnya dimulai.

### Jaminan Memory Safety

```
✅ Semua buffer menggunakan Vec<u8> ownership
✅ Tidak ada raw pointer di runtime path
✅ Tidak ada global mutable state (tidak ada static mut)
✅ Zero-copy via move semantics untuk VIDEO-MP4 dan IMAGE-JPG
✅ Option<T> / Result<T,E> — tidak ada null pointer
```

### Dispatch Operasi

```rust
// COMPRESS — didelegasikan ke cobol-protocol-v153
fn dispatch_compress(buf: Vec<u8>) -> Result<Vec<u8>, CnfError> {
    cobol_protocol_v153::compress_l1_l3(buf)
}

// VERIFY-INTEGRITY — didelegasikan ke cnf-security
fn dispatch_verify(buf: &[u8]) -> String {
    cnf_security::sha256_hex(buf)
}
```

---

## Security Model

### Integrity Verification

`VERIFY-INTEGRITY` menghitung SHA-256 digest dari buffer target via `cnf-security`. Digest bersifat deterministik: input yang sama selalu menghasilkan digest yang sama.

- **Algoritma:** SHA-256 (256-bit digest)
- **Output:** Hex-encoded string untuk logging dan audit trail
- **Isolasi:** Seluruh operasi crypto terkurung dalam `cnf-security` — tidak ada crate lain yang boleh melakukan hashing

### Security Boundary

```
cnf-security  ←──── SATU-SATUNYA lokasi operasi kriptografi
      │
      └── sha256_hex(&[u8]) → String
```

Jika ada kebutuhan operasi crypto baru, implementasinya harus masuk ke `cnf-security`, bukan di crate lain.

---

## Panduan Kontribusi

### Alur Kerja

```bash
# 1. Buat branch dari main
git checkout -b feat/nama-fitur

# 2. Kembangkan dengan TDD — tulis test dulu
cargo test -p <crate-yang-relevan>

# 3. Format dan lint sebelum commit
cargo fmt --all
cargo clippy --all -- -D warnings

# 4. Pastikan semua test lulus
cargo test --all

# 5. Buat pull request ke main
```

### Checklist sebelum PR

- [ ] `cargo fmt --all` tanpa perubahan
- [ ] `cargo clippy --all -- -D warnings` tanpa warning
- [ ] `cargo test --all` semua lulus
- [ ] Test baru ditambahkan untuk setiap perubahan fungsional
- [ ] Komentar menjelaskan **mengapa** (intent arsitektural), bukan **apa**
- [ ] Tidak ada `static mut` atau global mutable state yang ditambahkan
- [ ] Tidak ada modifikasi pada `cobol-protocol-v153`

---

## Menambah Operasi Baru

Ikuti tujuh langkah ini secara berurutan. Setiap langkah memiliki test yang harus ditulis sebelum implementasi.

**Contoh: menambah operasi `TRANSCODE`**

### Langkah 1 — Lexer: daftarkan keyword baru

```rust
// crates/cnf-compiler/src/lexer.rs
pub enum Token {
    // ... token yang ada ...
    Compress,
    VerifyIntegrity,
    Transcode,       // ← tambahkan di sini
}

fn keyword_to_token(s: &str) -> Option<Token> {
    match s {
        "COMPRESS"          => Some(Token::Compress),
        "VERIFY-INTEGRITY"  => Some(Token::VerifyIntegrity),
        "TRANSCODE"         => Some(Token::Transcode),  // ← dan di sini
        _                   => None,
    }
}
```

### Langkah 2 — AST: tambahkan node variant

```rust
// crates/cnf-compiler/src/ast.rs
pub enum ProcedureStatement {
    Compress  { target: String },
    Verify    { target: String },
    Transcode { target: String, format: String },  // ← variant baru
}
```

### Langkah 3 — Parser: tangani token baru

```rust
// crates/cnf-compiler/src/parser.rs
Token::Transcode => {
    let target = self.expect_identifier()?;
    let format = self.expect_identifier()?;
    Ok(ProcedureStatement::Transcode { target, format })
}
```

### Langkah 4 — IR: lower AST node ke instruksi

```rust
// crates/cnf-compiler/src/ir.rs
pub enum Instruction {
    Compress(String),
    Verify(String),
    Transcode(String, String),  // ← instruksi baru
}

fn lower_statement(stmt: ProcedureStatement) -> Instruction {
    match stmt {
        ProcedureStatement::Transcode { target, format } =>
            Instruction::Transcode(target, format),
        // ...
    }
}
```

### Langkah 5 — Runtime: implementasi dispatch

```rust
// crates/cnf-runtime/src/runtime.rs
Instruction::Transcode(target, format) => {
    let buf = self.get_buffer(&target)?;
    let result = cnf_transcode::transcode(buf, &format)?;
    self.set_buffer(target, result);
    Ok(())
}
```

### Langkah 6 — Test

```rust
// crates/cnf-compiler/tests/transcode_test.rs
#[test]
fn test_transcode_keyword_lexed() { /* ... */ }

#[test]
fn test_transcode_ast_node_built() { /* ... */ }

#[test]
fn test_transcode_ir_lowered_correctly() { /* ... */ }

#[test]
fn test_invalid_transcode_missing_format_fails() { /* ... */ }
```

### Langkah 7 — Dokumentasi

Tambahkan `TRANSCODE` (atau operasi lain yang anda tambahkan, misalnya `ENCRYPT`/`DECRYPT`, `FILTER`, `MERGE`, dll.) ke tabel operasi di `docs/specification.md` dan buat contoh `.cnf` di `examples/.`.

---

## Menambah Data Type Baru

**Contoh: menambah tipe `AUDIO-WAV`**

```bash
# Urutan perubahan:
# 1. Lexer   — tambahkan token AudioWav
# 2. AST     — tambahkan variant DataType::AudioWav
# 3. Parser  — kenali keyword "AUDIO-WAV" dalam DATA division
# 4. Runtime — definisikan perilaku buffer (ukuran, alignment)
# 5. Test    — deklarasi valid, tipe tidak dikenal harus gagal
```

Tidak ada perubahan yang diperlukan di IR untuk data type baru — IR merekam nama variabel, bukan tipenya secara eksplisit. Tipe divalidasi saat runtime dispatch.

---

## Konvensi Kode

### Rust Style

```rust
// ✅ Benar: eksplisit, readable, tidak pintar
fn compress_buffer(input: Vec<u8>) -> Result<Vec<u8>, CnfError> {
    // Mendelegasikan ke protocol layer — runtime tidak mengandung logic kompresi
    cobol_protocol_v153::compress_l1_l3(input)
        .map_err(CnfError::CompressionFailed)
}

// ❌ Salah: implisit, sulit dibaca
fn compress_buffer(i: Vec<u8>) -> Result<Vec<u8>, CnfError> {
    Ok(cobol_protocol_v153::compress_l1_l3(i)?)
}
```

### Komentar

```rust
// ✅ Benar: jelaskan intent arsitektural
// Protokol dipisahkan dari runtime agar L1-L3 dapat diupdate
// secara independen tanpa menyentuh scheduling logic.
fn dispatch_compress(buf: Vec<u8>) -> Result<Vec<u8>, CnfError> { ... }

// ❌ Salah: mengulang apa yang kode sudah jelaskan sendiri
// Memanggil fungsi compress dengan buffer sebagai argumen
fn dispatch_compress(buf: Vec<u8>) -> Result<Vec<u8>, CnfError> { ... }
```

### Error Handling

```rust
// ✅ Selalu gunakan Result<T, E> — tidak ada unwrap() di production path
fn parse_division(&mut self) -> Result<Division, ParseError> {
    match self.current_token() {
        Token::Identification => self.parse_identification(),
        token => Err(ParseError::UnexpectedToken {
            expected: "IDENTIFICATION DIVISION",
            got: token.clone(),
            line: self.current_line(),
        }),
    }
}

// ❌ Tidak pernah di production path
let token = self.next().unwrap();
```

### Yang Tidak Boleh Dilakukan

```rust
// ❌ Global mutable state
static mut COUNTER: u64 = 0;

// ❌ Raw pointer di runtime path
let ptr = buf.as_ptr() as *mut u8;

// ❌ Modifikasi cobol-protocol-v153
// Crate ini CORE-FROZEN — tidak ada pengecualian

// ❌ Panic sebagai error handling normal
panic!("sesuatu tidak beres");  // gunakan Result

// ❌ Clone buffer yang bisa di-move
let copy = buf.clone();  // gunakan move semantics jika memungkinkan
```

---

## Troubleshooting

### `cargo check` gagal: "rustc not found"

```bash
# Rust belum terinstall atau PATH belum dikonfigurasi
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### `cargo test` gagal: "error[E0433]: failed to resolve"

Dependency antar crate belum terdaftar di `Cargo.toml` workspace. Periksa:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/cnf-compiler",
    "crates/cnf-runtime",
    "crates/cnf-security",
    "crates/cobol-protocol-v153",
]
```

### Parser error: "expected IDENTIFICATION DIVISION"

Division tidak ditulis dalam urutan yang benar. Urutan wajib:

```
IDENTIFICATION DIVISION.   ← harus pertama
ENVIRONMENT DIVISION.
DATA DIVISION.
PROCEDURE DIVISION.        ← harus terakhir
```

### Parse error: "expected quoted string in ENVIRONMENT"

Nilai di `ENVIRONMENT DIVISION` harus selalu dibungkus tanda kutip ganda:

```cobol
# ❌ Salah
OS Linux.

# ✅ Benar
OS "Linux".
```

### `cargo clippy` memunculkan warning tentang `unwrap()`

Ganti dengan error handling eksplisit:

```rust
// Sebelum
let val = some_option.unwrap();

// Sesudah
let val = some_option.ok_or(CnfError::MissingValue)?;
```

---

## Roadmap

| Versi | Target | Item |
|---|---|---|
| **v0.2** | Q2 2026 | Fix Rust toolchain di dev container; error code catalog (CNF-E001…) |
| **v0.3** | Q3 2026 | Formal EBNF grammar; PROCEDURE argument validation |
| **v1.0** | Q4 2026 | DAG layer semantics; integration test suite; production readiness |
| **v1.1** | Q1 2027 | LSP server untuk .cnf; syntax highlighting; 5+ operasi baru |
| **v2.0** | 2027 | GPU dispatch; parallel scheduler; AUDIO-WAV, CSV-TABLE, BINARY-BLOB |

Detail lengkap tersedia di [`docs/specification.md`](docs/specification.md).

---

## License

```
Copyright © 2026 Nafal Faturizki

Permission is granted to read and reference this specification.
Implementation requires a separate license.
```

Untuk informasi lisensi implementasi, hubungi Nafal Faturizki secara langsung.

---

<div align="center">
  <sub>Copyright © 2026 Nafal Faturizki · Built with Rust · Fail-Fast Design · Zero Global State</sub>
</div>
