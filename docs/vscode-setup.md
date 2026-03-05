# VS Code Setup for CENTRA-NF LSP Server

This guide explains how to configure and debug the CENTRA-NF Language Server Protocol (LSP) server in VS Code.

## Prerequisites

1. **Rust toolchain** installed (1.70+)
2. **VS Code** with CodeLLDB extension (`vadimcn.vscode-lldb`)
3. **rust-analyzer** extension (optional but recommended)

## Installation

### Step 1: Install Extensions
VS Code will automatically recommend extensions when opening this workspace:
- `rust-lang.rust-analyzer` - Rust language support
- `vadimcn.vscode-lldb` - Debugger for Rust
- `tamasfe.even-better-toml` - TOML syntax highlighting

### Step 2: Build Debug Binary
```bash
cargo build --package centra-nf-lsp
```

Output: `/workspaces/CENTRA-NF/target/debug/centra-nf-lsp`

### Step 3: Build Release Binary (Optional)
```bash
cargo build --package centra-nf-lsp --release
```

Output: `/workspaces/CENTRA-NF/target/release/centra-nf-lsp`

## Debug Configurations

The workspace includes three launch configurations in `.vscode/launch.json`:

### 1. LSP Server Debug
```bash
Debug Name: "LSP Server Debug"
Binary: target/debug/centra-nf-lsp
Console: Integrated Terminal
Requires: CodeLLDB extension
```

**How to use:**
1. Press `F5` or go to Run → Start Debugging
2. Select "LSP Server Debug" from dropdown
3. Server will start with debugger attached
4. Set breakpoints in source code
5. Connect with LSP client to hit breakpoints

### 2. LSP Server (Release)
```bash
Debug Name: "LSP Server (Release)"
Binary: target/release/centra-nf-lsp
Console: Integrated Terminal
```

**How to use:**
1. Select "LSP Server (Release)" from debug dropdown
2. Useful for performance testing
3. Optimized build (slower startup but faster execution)

### 3. Run LSP Server (stdio)
```bash
Debug Name: "Run LSP Server (stdio)"
Type: Node
Console: Integrated Terminal
```

## Quick Start

### Method 1: Direct Execution (Testing)
```bash
# Build the server
cargo build --package centra-nf-lsp

# Run directly (listen on stdin for JSON-RPC messages)
./target/debug/centra-nf-lsp
```

Then in another terminal, send JSON-RPC messages:
```bash
# Initialize request
echo -e 'Content-Length: 95\r\n\r\n{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":1234}}' | ./target/debug/centra-nf-lsp
```

### Method 2: VS Code Debugging (Development)
1. Open `.vscode/launch.json` in VS Code
2. Press `F5` to start debugging
3. Confirm launch configuration "LSP Server Debug"
4. Server runs with breakpoints enabled

## Integration with Editor

To use the LSP server in VS Code (requires LSP client extension):

### Option A: Manual Client Configuration
Create an LSP client extension that points to the server binary:
```json
{
  "serverPath": "${workspaceFolder}/target/debug/centra-nf-lsp",
  "languages": [
    {
      "languageId": "cnf",
      "scheme": "file"
    }
  ]
}
```

### Option B: CLI Tool (Current)
The server can be used via CLI for single-file compilation:
```bash
centra-nf-cli compile --file example.cnf
```

This will show diagnostics in terminal format.

## Available VS Code Tasks

Configured tasks can be run via `Ctrl+Shift+B` or Terminal → Run Task:

1. **cargo-build-lsp-debug** - Build debug binary
2. **cargo-build-lsp-release** - Build optimized binary
3. **cargo-test-lsp** - Run LSP package tests
4. **cargo-test-all** - Run all workspace tests

## Debugging Tips

### Setting Breakpoints
1. Open source file in `crates/centra-nf-lsp/src/(handler|publisher|server).rs`
2. Click line number to set breakpoint (red dot appears)
3. Start debugging with `F5`
4. When LSP message arrives at that code, execution pauses

### Inspecting Variables
During debugging pause:
- Hover over variables to see current value
- Use Debug Console to evaluate expressions
- Right-click variables → "Copy Value" or "Add to Watch"

### Viewing Call Stack
- In Run panel on left sidebar
- Shows function call chain
- Click frame to jump to that location in code

### Console Output
- Eprintln! messages appear in Debug Console (stderr)
- Use `eprintln!("🔍 Debug info: {:?}", variable);` for inspection

Example in code:
```rust
fn handle_hover(&self, req: &Request) -> Result<Response, String> {
    eprintln!("🔍 Hover request: {:?}", req);  // Visible in debug console
    // ...
}
```

## Common Issues

### Issue: Server doesn't start
- **Check**: Did you build the binary first? Run `cargo build --package centra-nf-lsp`
- **Check**: Is the binary path set correctly in `launch.json`?

### Issue: Breakpoints not being hit
- **Check**: Are you using debug binary, not release?
- **Check**: Is the source file saved and matches compiled version?
- **Workaround**: Add `eprintln!` statements instead

### Issue: "Binary not executable"
```bash
chmod +x target/debug/centra-nf-lsp
```

### Issue: CodeLLDB extension not found
- Install from VS Code Extensions: `vadimcn.vscode-lldb`
- Or run: `code --install-extension vadimcn.vscode-lldb`

## Testing the Server

### Protocol Compliance Test
```bash
cargo test --package centra-nf-lsp --test integration_tests
```

### Unit Tests
```bash
cargo test --package centra-nf-lsp --lib
```

### All Tests (Including Compiler)
```bash
cargo test --all
```

## Next Steps

1. **Create LSP Client Extension** - Build VS Code extension that uses this server
2. **Editor Integration** - Configure syntax highlighting for `.cnf` files
3. **Performance Testing** - Profile with `perf` or `flamegraph`
4. **Additional Features** - Implement rename, references, symbol resolution

## Resources

- [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [VS Code Debugging Guide](https://code.visualstudio.com/docs/editor/debugging)
- [CodeLLDB Documentation](https://github.com/vadimcn/vscode-lldb)
- [CENTRA-NF LSP Features](./lsp-features.md)
