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

    // Markdown text content
    let text_content = r#"
# My VTF Document

Hello, **world**!
This is *italic* text.

- Item 1
- Item 2

[button:Click me]
[checkbox:Do task?][ ]
[spoiler]This is hidden[/spoiler]
"#;

    // Metadata
    let metadata = Metadata {
        title: "My First VTF with Markdown".to_string(),
        author: "Skye".to_string(),
    };

    let metadata_json = to_string_pretty(&metadata).unwrap();

    // Create .vtf file
    let file = File::create(filename).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "VTFFORMAT 1.0").unwrap();
    writeln!(writer, "TEXT_START\n{}\nTEXT_END", text_content).unwrap();
    writeln!(writer, "META_START\n{}\nMETA_END", metadata_json).unwrap();

    println!("✅ Created: {}", filename);
}