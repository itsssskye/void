use std::fs::File;
use std::io::{Write, BufWriter, Read};
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize)]
struct Metadata {
    title: String,
    author: String,
}

// Function to read a file and convert it to Base64
fn embed_file_base64(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file for embedding");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    general_purpose::STANDARD.encode(&buffer)
}

fn main() {
    let filename = "example.vtf";

    // Markdown text content
    let text_content = r#"
# Introduction

Welcome to **Void Text Format**.[1]

- Item A
- Item B

# Details

Go back to [Introduction](#introduction)

[1]: This is a footnote example.
"#;

    // Metadata
    let metadata = Metadata {
        title: "My First VTF with Markdown + Embedded File".to_string(),
        author: "Skye".to_string(),
    };
    let metadata_json = to_string_pretty(&metadata).unwrap();

    // File to embed (example: image)
    let embedded_file_path = "example.png"; // replace with your file
    let embedded_file_base64 = embed_file_base64(embedded_file_path);

    // Create .vtf file
    let file = File::create(filename).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // Write sections
    writeln!(writer, "VTFFORMAT 1.0").unwrap();
    writeln!(writer, "TEXT_START\n{}\nTEXT_END", text_content).unwrap();
    writeln!(writer, "META_START\n{}\nMETA_END", metadata_json).unwrap();

    writeln!(writer, "BIN_START").unwrap();
    writeln!(writer, "{}:{}", embedded_file_path, embedded_file_base64).unwrap();
    writeln!(writer, "BIN_END").unwrap();

    println!("✅ Created: {} with embedded file", filename);
}