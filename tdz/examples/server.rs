//! Todozi Server Demo
//!
//! This example demonstrates how to start the Todozi REST API server
//! on port 8636 (TODO in dial language) and interact with it.

use todozi::{init, server::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the todozi system
    init()?;
    
    println!("🚀 Starting Todozi Server Demo");
    println!("📡 Port: 8636 (TODO in dial language!)");
    println!();
    
    // Start the server
    start_server(Some("127.0.0.1".to_string()), Some(8636)).await?;
    
    Ok(())
}
