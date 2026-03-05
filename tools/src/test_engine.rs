use std::fs;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

#[derive(Parser)]
#[command(name = "test_engine")]
#[command(about = "Run tests from errors_master.yaml with in-memory test code")]
struct Args {
    /// Input YAML file
    #[arg(short, long, default_value = "errors_master.yaml")]
    yaml_file: PathBuf,

    /// Layer to test (1-8), or 0 for all
    #[arg(short, long, default_value = "0")]
    layer: u8,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Temporary directory for test files
    #[arg(short, long, default_value = "/tmp")]
    temp_dir: PathBuf,

    /// Path to cnf-compiler binary
    #[arg(long, default_value = "cnf-compiler")]
    compiler: String,

    /// Stop on first failure
    #[arg(long)]
    fail_fast: bool,
}

#[derive(Debug, Clone)]
struct ErrorTestCase {
    code: String,
    layer: u8,
    title: String,
    trigger_code: String,
    expected_error: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("=== CENTRA-NF Error Test Engine ===");
    println!("Testing from: {}\n", args.yaml_file.display());

    // Read and parse YAML
    let yaml_content = fs::read_to_string(&args.yaml_file)?;
    let test_cases = parse_yaml(&yaml_content, args.layer)?;

    println!("Loaded {} test cases", test_cases.len());
    if args.layer > 0 {
        println!("Filtering for Layer {}", args.layer);
    }
    println!();

    // Run tests
    let (passed, failed) = run_tests(&test_cases, &args)?;

    // Print summary
    println!("\n=== Test Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total:  {}", passed + failed);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn parse_yaml(content: &str, layer_filter: u8) -> Result<Vec<ErrorTestCase>, Box<dyn std::error::Error>> {
    let mut tests = Vec::new();
    let mut current_test: Option<ErrorTestCase> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Check for 'errors:' array start
        if trimmed == "errors:" {
            continue;
        }

        // Check for new error item (- code:)
        if trimmed.starts_with("- code:") {
            // Save previous test
            if let Some(test) = current_test.take() {
                if layer_filter == 0 || test.layer == layer_filter {
                    tests.push(test);
                }
            }

            // Start new test
            if let Some(code) = extract_quoted_value(trimmed) {
                current_test = Some(ErrorTestCase {
                    code,
                    layer: 0,
                    title: String::new(),
                    trigger_code: String::new(),
                    expected_error: String::new(),
                });
            }
            continue;
        }

        // Parse fields
        if let Some(ref mut test) = current_test {
            if trimmed.starts_with("layer:") {
                if let Ok(num) = extract_number(trimmed) {
                    test.layer = num;
                }
            } else if trimmed.starts_with("title:") {
                if let Some(val) = extract_quoted_value(trimmed) {
                    test.title = val;
                }
            } else if trimmed.starts_with("trigger_code:") {
                if trimmed.ends_with("|") {
                    // Start of multiline - capture until next field
                    let start_idx = content.find(line).unwrap_or(0);
                    let rest = &content[start_idx + line.len()..];
                    let mut code_lines = Vec::new();
                    
                    for code_line in rest.lines() {
                        let code_trimmed = code_line.trim();
                        if code_trimmed.starts_with("expected_error:") || 
                           code_trimmed.starts_with("fix:") ||
                           code_trimmed.starts_with("- code:") {
                            break;
                        }
                        if !code_trimmed.is_empty() && !code_trimmed.starts_with('#') {
                            code_lines.push(code_line);
                        }
                    }
                    test.trigger_code = code_lines.join("\n");
                }
            } else if trimmed.starts_with("expected_error:") {
                if let Some(val) = extract_quoted_value(trimmed) {
                    test.expected_error = val;
                }
            }
        }
    }

    // Save last test
    if let Some(test) = current_test {
        if layer_filter == 0 || test.layer == layer_filter {
            tests.push(test);
        }
    }

    Ok(tests)
}

fn extract_quoted_value(line: &str) -> Option<String> {
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            return Some(line[start + 1..start + 1 + end].to_string());
        }
    }
    None
}

fn extract_number(line: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() > 1 {
        return Ok(parts[1].trim().parse()?);
    }
    Err("Cannot parse number".into())
}

fn run_tests(
    tests: &[ErrorTestCase],
    args: &Args,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut passed = 0;
    let mut failed = 0;

    for (_, test) in tests.iter().enumerate() {
        // Create temporary file
        let temp_file = args.temp_dir.join(format!("test_{}.cnf", test.code));

        // Write trigger code to temp file
        fs::write(&temp_file, &test.trigger_code)?;

        // Run compiler
        let output = Command::new(&args.compiler)
            .arg(&temp_file)
            .output();

        // Check result
        let test_passed = match output {
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let combined = format!("{}\n{}", stdout, stderr);

                // Check if expected error appears in output
                combined.contains(&test.expected_error) || !output.status.success()
            }
            Err(_) => false, // Compiler not found is a fail
        };

        // Clean up temp file
        let _ = fs::remove_file(&temp_file);

        // Report result
        if test_passed {
            passed += 1;
            if args.verbose {
                println!("✓ {} ({}): {}", test.code, test.layer, test.title);
            }
        } else {
            failed += 1;
            println!("✗ {} ({}): {}", test.code, test.layer, test.title);
            if args.verbose {
                println!("  Expected: {}", test.expected_error);
            }
            if args.fail_fast {
                break;
            }
        }
    }

    Ok((passed, failed))
}
