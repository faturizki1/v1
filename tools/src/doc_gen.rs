use std::fs;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "doc_gen")]
#[command(about = "Generate documentation from errors_master.yaml")]
struct Args {
    /// Input YAML file
    #[arg(short, long, default_value = "errors_master.yaml")]
    input: PathBuf,

    /// Output markdown file
    #[arg(short, long, default_value = "docs/error-codes.md")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read YAML
    let yaml_content = fs::read_to_string(&args.input)?;

    // Parse YAML (simple line-by-line parsing for demo)
    let mut markdown = String::from(
        "# CENTRA-NF Error Code Reference\n\n\
         Auto-generated from errors_master.yaml\n\n\
         All errors in CENTRA-NF are categorized by error code.\n\n\
         ---\n\n\
         ## Error Code Format\n\n\
         `CNF-XYYY`\n\n\
         - `X` = Layer (L=Lexer, P=Parser, I=IR, R=Runtime, S=Security, O=Protocol, C=CLI, X=LSP)\n\
         - `YYY` = Sequential number (001-999)\n\n\
         ---\n\n"
    );

    // Extract error sections by layer
    let layers = vec![
        (1, "Lexer", "L"),
        (2, "Parser", "P"),
        (3, "IR", "I"),
        (4, "Runtime", "R"),
        (5, "Security", "S"),
        (6, "Protocol", "O"),
        (7, "CLI", "C"),
        (8, "LSP", "X"),
    ];

    for (layer_num, layer_name, prefix) in layers {
        // Build section header
        markdown.push_str(&format!("## {} Errors ({}***)\n\n", layer_name, prefix));
        markdown.push_str("| Code | Title | Description | Fix |\n");
        markdown.push_str("|------|-------|-------------|-----|\n");

        // Find errors for this layer from YAML content
        let mut in_layer = false;
        for line in yaml_content.lines() {
            if line.contains(&format!("layer: {}", layer_num)) {
                in_layer = true;
            }

            if in_layer && line.contains("code: ") {
                // Extract code
                if let Some(code_part) = line.split("code: \"").nth(1) {
                    if let Some(code) = code_part.split('"').next() {
                        // Build a minimal entry for demo
                        let title = "Error";
                        let desc = "See trigger_code for details";
                        let fix = "Review error and fix per description";
                        markdown.push_str(&format!(
                            "| {} | {} | {} | {} |\n",
                            code, title, desc, fix
                        ));
                    }
                }
                in_layer = false;
            }
        }
        markdown.push_str("\n");
    }

    markdown.push_str("---\n\n");
    markdown.push_str("*This documentation is auto-generated from errors_master.yaml*\n");

    // Write to file
    fs::write(&args.output, &markdown)?;
    println!("✓ Documentation generated: {}", args.output.display());

    Ok(())
}
