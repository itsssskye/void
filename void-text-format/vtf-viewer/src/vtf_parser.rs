use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub author: String,
    pub created_at: String,
    pub word_count: usize,
    pub char_count: usize,
}

#[derive(Debug)]
pub struct ParsedVtf {
    pub version: String,
    pub text: String,
    pub metadata: Metadata,
    pub embedded_files: HashMap<String, String>, // filename -> base64
}

pub fn parse_vtf(input: &str) -> Result<ParsedVtf, String> {
    let version = input
        .lines()
        .next()
        .ok_or("Missing version line")?
        .trim()
        .replace("VTFFORMAT ", "");

    // Extract sections
    let text = extract_block(input, "TEXT_START", "TEXT_END")?;
    let meta_raw = extract_block(input, "META_START", "META_END")?;
    let bin_raw = extract_block(input, "BIN_START", "BIN_END")?;

    // Parse metadata JSON
    let metadata: Metadata =
        serde_json::from_str(&meta_raw).map_err(|_| "Invalid metadata JSON")?;

    // Parse embedded files
    let mut embedded_files = HashMap::new();
    for line in bin_raw.lines() {
        if let Some((name, b64)) = line.split_once(':') {
            embedded_files.insert(name.to_string(), b64.to_string());
        }
    }

    Ok(ParsedVtf {
        version,
        text,
        metadata,
        embedded_files,
    })
}

fn extract_block(input: &str, start: &str, end: &str) -> Result<String, String> {
    let start_tag = input.find(start).ok_or(format!("Missing {}", start))?;
    let end_tag = input.find(end).ok_or(format!("Missing {}", end))?;

    let block = &input[start_tag + start.len()..end_tag];

    Ok(block.trim().to_string())
}