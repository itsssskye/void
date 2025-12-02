use std::fs::File;
use std::io::{Write, BufWriter};
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize)]
struct Metadata {
    title: String,
    author: String,
}

fn main() {
    let filename = "example.vtf";

    let text_content = "Hello, .vtf world! This is Skye's format.";

    let metadata = Metadata {
        title: "My First VTF".to_string(),
        author: "Skye".to_string(),
    };

    let metadata_json = to_string_pretty(&metadata).unwrap();

    let file = File::create(filename).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "VTFFORMAT 1.0").unwrap();
    writeln!(writer, "TEXT_START\n{}\nTEXT_END", text_content).unwrap();
    writeln!(writer, "META_START\n{}\nMETA_END", metadata_json).unwrap();

    println!("✅ Created: {}", filename);
}