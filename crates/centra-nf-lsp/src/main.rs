//! CENTRA-NF LSP Server
//!
//! Provides language server protocol support for .cnf files.
//! Runs over stdio with JSON-RPC 2.0 protocol.

use centra_nf_lsp::{JsonRpcIO, MessageHandler};

fn main() {
    eprintln!("🚀 CENTRA-NF LSP Server v0.1.0");
    eprintln!("Waiting for client initialization...\n");

    let mut io = JsonRpcIO::new();
    let handler = MessageHandler::new();

    loop {
        match io.read_message() {
            Ok(None) => {
                eprintln!("📭 EOF: Client disconnected");
                break;
            }
            Ok(Some(message)) => {
                match handler.handle_message(message, &mut io) {
                    Ok(Some(response)) => {
                        if let Err(e) = io.send_response(&response) {
                            eprintln!("❌ Error sending response: {}", e);
                            break;
                        }
                        eprintln!("✉️  Response sent (id={})\n", response.id);
                    }
                    Ok(None) => {
                        // Notification processed, no response
                        eprintln!();
                    }
                    Err(e) => {
                        eprintln!("❌ Handler error: {}\n", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading message: {}", e);
                break;
            }
        }
    }

    eprintln!("👋 Server shutdown");
}
