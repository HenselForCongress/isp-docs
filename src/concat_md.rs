// src/concat_md.rs

use std::fs::{self, File};
use std::io::{self, Write};
use regex::Regex;

// Function to clean individual markdown content
fn clean_markdown(content: &str) -> String {
    let mut cleaned = content.to_string();
    // Strip HTML comments
    let comment_re = Regex::new(r"<!--.*?-->").unwrap();
    cleaned = comment_re.replace_all(&cleaned, "").to_string();

    // Remove extra whitespace
    cleaned = cleaned.lines().map(|line| line.trim()).collect::<Vec<_>>().join("\n");

    // Add more cleanup rules if necessary
    cleaned
}

pub fn main() -> io::Result<()> {
    let input_dir = "docs";
    let output_file = "all_docs.md";

    let mut outfile = File::create(output_file)?;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
            let file_content = fs::read_to_string(entry.path())?;
            let cleaned_content = clean_markdown(&file_content);
            let title = entry.file_name().into_string().unwrap()
                .replace(".md", "")
                .replace("-", " ")
                .to_uppercase();
            writeln!(outfile, "# {}\n", title)?;
            writeln!(outfile, "{}\n", cleaned_content)?;
            writeln!(outfile, "---\n")?;
        }
    }

    println!("All Markdown files concatenated into {}", output_file);
    Ok(())
}
