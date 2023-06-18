use std::io::{self, BufRead};
use std::fs;

fn main() {
    // Read the file path from stdin
    let stdin = io::stdin();
    println!("Enter the path to the file:");
    let file_path = stdin.lock().lines().next().unwrap().unwrap();

    // Read the file content
    let file_content = fs::read_to_string(&file_path)
        .expect("Failed to read the file.");

    // Scrape URLs from the file content
    let urls = scrape_urls(&file_content);

    // Print the scraped URLs
    println!("Scraped URLs:");
    for url in urls {
        println!("{}", url);
    }
}

fn scrape_urls(content: &str) -> Vec<String> {
    let pattern = r#"(?i)\b((?:https?://|www\.)\S+)\b"#;
    let regex = regex::Regex::new(pattern).unwrap();
    let mut urls = Vec::new();

    for capture in regex.captures_iter(content) {
        let matched_string = capture.get(0).unwrap().as_str().to_string();
        urls.push(matched_string);
    }

    urls
}
