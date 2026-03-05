use clap::{Parser, ValueEnum};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

// ============================================================================
// CLI Arguments
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "gen_errors")]
#[command(about = "Generate CENTRA-NF error codes, test files, and documentation")]
#[command(version = "0.1.0")]
struct Args {
    /// Layer designation (1-8: Lexer, Parser, IR, Runtime, Security, Protocol, CLI, LSP)
    #[arg(short, long, default_value = "1")]
    layer: u8,

    /// Error category: Syntax, Semantic, or Runtime
    #[arg(short, long, value_enum, default_value = "syntax")]
    category: ErrorCategory,

    /// Number of errors to generate
    #[arg(short = 'n', long, default_value = "100")]
    count: usize,

    /// Output directory for test files (relative to workspace root)
    #[arg(short, long, default_value = "tests/ui/fail")]
    test_dir: PathBuf,

    /// Documentation file to update
    #[arg(short, long, default_value = "docs/error-codes.md")]
    doc_file: PathBuf,

    /// Generate dry-run only (no file writes)
    #[arg(long)]
    dry_run: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum ErrorCategory {
    Syntax,
    Semantic,
    Runtime,
}

// ============================================================================
// Error Code Generator
// ============================================================================

const LAYER_NAMES: &[&str] = &["Lexer", "Parser", "IR", "Runtime", "Security", "Protocol", "CLI", "LSP"];

const KEYWORDS: &[&str] = &[
    "Invalid",
    "Missing",
    "Overflow",
    "Underflow",
    "Unexpected",
    "Illegal",
    "Malformed",
    "Unterminated",
    "Undefined",
    "Duplicate",
    "Mismatch",
    "Type",
    "Constraint",
    "Boundary",
    "State",
    "Order",
    "Syntax",
    "Semantic",
    "Unmatched",
    "Expected",
];

const DATA_TYPES: &[&str] = &[
    "BINARY-BLOB",
    "VIDEO-MP4",
    "IMAGE-JPG",
    "FINANCIAL-DECIMAL",
    "AUDIO-WAV",
    "TEXT-UTF8",
    "DOCUMENT-PDF",
    "DATA-CSV",
];

const SUPPLEMENTARY: &[&str] = &[
    "in division structure",
    "in instruction sequence",
    "in variable declaration",
    "in expression",
    "in control flow",
    "in type annotation",
    "in indentation",
    "in encoding",
];

struct ErrorGenerator {
    layer: u8,
    category: ErrorCategory,
    count: usize,
    test_dir: PathBuf,
    doc_file: PathBuf,
    dry_run: bool,
    generated_codes: HashSet<String>,
}

impl ErrorGenerator {
    fn new(args: Args) -> Self {
        Self {
            layer: args.layer,
            category: args.category,
            count: args.count,
            test_dir: args.test_dir,
            doc_file: args.doc_file,
            dry_run: args.dry_run,
            generated_codes: HashSet::new(),
        }
    }

    fn layer_prefix(&self) -> char {
        match self.layer {
            1 => 'L',
            2 => 'P',
            3 => 'I',
            4 => 'R',
            5 => 'S',
            6 => 'O', // prOtocol
            7 => 'C',
            8 => 'X', // lsP (X untuk LSP)
            _ => '?',
        }
    }

    fn layer_name(&self) -> &'static str {
        LAYER_NAMES.get((self.layer - 1) as usize).unwrap_or(&"Unknown")
    }

    /// Generate all unique error codes dan metadata
    fn generate_errors(&mut self) -> Vec<ErrorMetadata> {
        let mut errors = Vec::new();
        let mut code_counter = 1;

        // Create permutation list dari keywords dan data types
        let all_terms: Vec<&str> = KEYWORDS.iter().chain(DATA_TYPES.iter()).copied().collect();

        // Generate permutasi: ambil 2 terms berbeda dari all_terms
        for combo in all_terms.iter().combinations(2).unique().take(self.count) {
            if errors.len() >= self.count {
                break;
            }

            let kw1 = combo[0];
            let kw2 = combo[1];
            let supplement = SUPPLEMENTARY[code_counter % SUPPLEMENTARY.len()];
            let error_code = format!("{}{}{:03}", self.layer_prefix(), self.layer, code_counter);
            let message = self.generate_message(kw1, kw2, supplement);

            errors.push(ErrorMetadata {
                code: error_code,
                message,
                example: self.generate_example(kw1, kw2),
                fix: self.generate_fix(kw1, kw2),
                layer_num: self.layer,
                category: format!("{:?}", self.category),
            });

            code_counter += 1;
        }

        // Fill remaining count with generated errors dari kombinasi lain
        while errors.len() < self.count {
            let kw = KEYWORDS[errors.len() % KEYWORDS.len()];
            let dt = DATA_TYPES[errors.len() / KEYWORDS.len() % DATA_TYPES.len()];
            let supplement = SUPPLEMENTARY[errors.len() % SUPPLEMENTARY.len()];

            let error_code = format!("{}{}{:03}", self.layer_prefix(), self.layer, code_counter);
            let message = self.generate_message(&kw, &dt, supplement);

            errors.push(ErrorMetadata {
                code: error_code,
                message,
                example: self.generate_example(&kw, &dt),
                fix: self.generate_fix(&kw, &dt),
                layer_num: self.layer,
                category: format!("{:?}", self.category),
            });

            code_counter += 1;
        }

        errors
    }

    fn generate_message(&self, keyword1: &str, keyword2: &str, supplement: &str) -> String {
        match self.layer {
            1 => format!(
                "{} {} character {} -- expected valid UTF-8 encoding",
                keyword1, keyword2, supplement
            ),
            2 => format!(
                "{} {} structure {} -- check division order",
                keyword1, keyword2, supplement
            ),
            3 => format!(
                "{} {} type conversion {} -- {} cannot be converted",
                keyword1, keyword2, supplement, keyword2
            ),
            4 => format!(
                "{} {} operation {} -- buffer mismatch",
                keyword1, keyword2, supplement
            ),
            5 => format!(
                "{} {} cryptographic {} -- determinism violation",
                keyword1, keyword2, supplement
            ),
            6 => format!(
                "{} {} protocol {} -- compression layer failed",
                keyword1, keyword2, supplement
            ),
            7 => format!(
                "{} {} command-line {} -- invalid argument format",
                keyword1, keyword2, supplement
            ),
            8 => format!(
                "{} {} language-server {} -- protocol mismatch",
                keyword1, keyword2, supplement
            ),
            _ => format!("Unknown error: {} {} {}", keyword1, keyword2, supplement),
        }
    }

    fn generate_example(&self, keyword1: &str, keyword2: &str) -> String {
        match self.layer {
            1 => format!(
                "IDENTIFICATION DIVISION.\n    PROGRAM \"test-{}\".\nEVIRONMENT DIVISION.\n    OS \"invalid™utf8-chars\".",
                keyword1.to_lowercase()
            ),
            2 => format!(
                "DATA DIVISION.\n    VIDEO-MP4 {}.\nIDENTIFICATION DIVISION.\n    PROGRAM \"{}\".",
                keyword2.to_uppercase(),
                keyword1.to_lowercase()
            ),
            3 => format!(
                "DATA DIVISION.\n    {} my_var.\nPROCEDURE DIVISION.\n    CONVERT my_var TO {}.",
                keyword2, keyword1
            ),
            4 => format!(
                "PROCEDURE DIVISION.\n    COMPRESS {} buffer.\n    VERIFY-INTEGRITY missing_buffer.",
                keyword2.to_lowercase()
            ),
            5 => format!(
                "PROCEDURE DIVISION.\n    ENCRYPT {{}} \"test-{}-data\".",
                keyword1.to_lowercase()
            ),
            6 => format!(
                "PROCEDURE DIVISION.\n    COMPRESS {}.\n    -- {} operation failed",
                keyword2, keyword1
            ),
            7 => format!("gen_errors --layer {} --category {} --count abc", self.layer, keyword1.to_lowercase()),
            _ => format!(
                "PROCEDURE DIVISION.\n    -- {} {} example",
                keyword1, keyword2
            ),
        }
    }

    fn generate_fix(&self, keyword1: &str, keyword2: &str) -> String {
        match self.layer {
            1 => format!(
                "Ensure source file uses valid UTF-8 encoding. Remove non-ASCII characters or use proper escaping."
            ),
            2 => format!(
                "Check division order: IDENTIFICATION → ENVIRONMENT → DATA → PROCEDURE. {} and {} must follow rules.",
                keyword1, keyword2
            ),
            3 => format!(
                "Type {} cannot be implicitly converted to {}. Use explicit type annotation or intermediate conversion.",
                keyword2, keyword1
            ),
            4 => format!(
                "Verify buffer '{}' is declared in DATA DIVISION before use in PROCEDURE DIVISION.",
                keyword2.to_lowercase()
            ),
            5 => format!(
                "Encryption operations must use deterministic keys/IVs. Check {} configuration.",
                keyword1.to_lowercase()
            ),
            6 => format!(
                "Compression {} failed with {}. Check protocol version compatibility.",
                keyword2.to_lowercase(),
                keyword1.to_lowercase()
            ),
            7 => format!(
                "Verify command-line argument format. {} must be valid {}.",
                keyword1, keyword2
            ),
            _ => format!(
                "Check {} {} configuration and try again.",
                keyword1.to_lowercase(),
                keyword2.to_lowercase()
            ),
        }
    }

    /// Generate .cnf test file dengan expected-error header
    fn generate_test_file(&self, error: &ErrorMetadata) -> String {
        format!(
            "// Test file for error code: {}\n// Expected error: {}\n// Category: {}\n// Layer: {} ({})\n\n{}\n",
            error.code,
            error.message,
            error.category,
            error.layer_num,
            self.layer_name(),
            error.example
        )
    }

    /// Tambah entry ke docs/error-codes.md
    fn generate_doc_entry(&self, error: &ErrorMetadata) -> String {
        format!(
            "| {} | {} | ```cnf\n{}\n``` | {} |",
            error.code, error.message, error.example, error.fix
        )
    }

    /// Write semua generated errors ke filesystem
    fn write_files(&self, errors: &[ErrorMetadata]) -> Result<(), Box<dyn std::error::Error>> {
        if self.dry_run {
            println!("=== DRY RUN: {} errors would be generated ===\n", errors.len());
            for error in errors.iter().take(5) {
                println!("Code: {}", error.code);
                println!("Message: {}", error.message);
                println!("---");
            }
            if errors.len() > 5 {
                println!("... and {} more errors\n", errors.len() - 5);
            }
            return Ok(());
        }

        // Ensure test directory exists
        fs::create_dir_all(&self.test_dir)?;

        // Write test .cnf files
        for error in errors {
            let file_name = format!("{}.cnf", error.code.to_lowercase());
            let file_path = self.test_dir.join(file_name);
            let content = self.generate_test_file(error);
            fs::write(&file_path, content)?;
            println!("✓ Created test file: {}", file_path.display());
        }

        // Update documentation
        self.update_documentation(errors)?;

        Ok(())
    }

    fn update_documentation(&self, errors: &[ErrorMetadata]) -> Result<(), Box<dyn std::error::Error>> {
        // Read existing docs
        let mut doc_content = fs::read_to_string(&self.doc_file)?;

        // Find or create section for this layer
        let section_header = format!(
            "## Layer {}: {} Errors\n\n",
            self.layer, self.layer_name()
        );

        let table_header = "| Code | Message | Example | Fix |\n|------|---------|---------|-----|\n";

        // Generate table entries
        let entries: Vec<String> = errors.iter().map(|e| self.generate_doc_entry(e)).collect();
        let new_section = format!("{}{}{}\n", section_header, table_header, entries.join("\n"));

        // Check if section exists
        if doc_content.contains(&section_header) {
            // Find existing section and replace it
            if let Some(start_pos) = doc_content.find(&section_header) {
                // Find next section or end of file
                let after_header = start_pos + section_header.len();
                let next_section_pos = if let Some(pos) = doc_content[after_header..].find("\n## ")
                {
                    after_header + pos
                } else {
                    doc_content.len()
                };

                doc_content.replace_range(start_pos..next_section_pos, &new_section);
            }
        } else {
            // Append new section before any final comments
            doc_content.push('\n');
            doc_content.push_str(&new_section);
        }

        fs::write(&self.doc_file, doc_content)?;
        println!("✓ Updated documentation: {}", self.doc_file.display());

        Ok(())
    }
}

// ============================================================================
// Error Metadata
// ============================================================================

#[derive(Clone, Debug)]
struct ErrorMetadata {
    code: String,
    message: String,
    example: String,
    fix: String,
    layer_num: u8,
    category: String,
}

// ============================================================================
// Main
// ============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validation
    if args.layer < 1 || args.layer > 8 {
        eprintln!("Error: Layer must be between 1 and 8");
        std::process::exit(1);
    }

    if args.count == 0 {
        eprintln!("Error: Count must be greater than 0");
        std::process::exit(1);
    }

    println!("=== CENTRA-NF Error Code Generator ===");
    println!("Layer: {} ({})", args.layer, LAYER_NAMES[(args.layer - 1) as usize]);
    println!("Category: {:?}", args.category);
    println!("Target count: {}", args.count);
    println!();

    let mut generator = ErrorGenerator::new(args);
    let errors = generator.generate_errors();

    println!("Generated {} error codes", errors.len());
    let sample_codes: String = errors
        .iter()
        .take(3)
        .map(|e| e.code.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    println!("Sample codes: {}", sample_codes);
    println!();

    generator.write_files(&errors)?;
    println!();
    println!("✓ Success! Errors generated and documentation updated.");

    Ok(())
}
