//! Export OpenAPI specification to JSON file
//!
//! Usage: cargo run --bin export_openapi

use std::fs;
use utoipa::OpenApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate OpenAPI spec
    let openapi = step8_rest_api::openapi::ApiDoc::openapi();
    
    // Serialize to pretty JSON
    let json = serde_json::to_string_pretty(&openapi)?;
    
    // Write to file
    fs::write("openapi.json", json)?;
    
    println!("✅ OpenAPI specification exported to openapi.json");
    println!("📄 You can now:");
    println!("   - Validate at https://editor.swagger.io/");
    println!("   - Generate clients with openapi-generator");
    println!("   - Share with API consumers");
    
    Ok(())
}
