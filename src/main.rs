// src/main.rs

mod concat_md;

extern crate reqwest;
extern crate select;
extern crate html2md;
extern crate serde;

use std::fs;
use std::error::Error;
use std::path::Path;
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use serde::Deserialize;
use serde_xml_rs::from_str;
use slug::slugify;
use std::env;
use futures::stream::{StreamExt, FuturesUnordered};
use tokio;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
struct Urlset {
    #[serde(rename = "url")]
    urls: Vec<Url>,
}

#[derive(Debug, Deserialize)]
struct Url {
    #[serde(rename = "loc")]
    loc: String,
}

async fn fetch_sitemap_xml(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

fn parse_sitemap(xml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let urlset: Urlset = from_str(xml)?;
    Ok(urlset.urls.into_iter().map(|url| url.loc).collect())
}

async fn fetch_page_html(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let attempts = 5;
    let mut delay = Duration::from_secs(1);

    for attempt in 1..=attempts {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return response.text().await.map_err(|e| Box::new(e) as _);
                } else if response.status().is_client_error() || response.status().is_server_error() {
                    println!("Error fetching URL (attempt {}): {}", attempt, url);
                    sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
            Err(e) => {
                println!("Error fetching URL (attempt {}): {}: {:?}", attempt, url, e);
                sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
        }
    }

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch page after multiple attempts")))
}

fn save_markdown(content: &str, slug: &str) -> Result<(), Box<dyn Error>> {
    let dir = Path::new("raw");
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    let file_path = dir.join(format!("{}.md", slug));
    fs::write(file_path, content)?;
    Ok(())
}

async fn process_url(client: &Client, url: String) -> Result<(), Box<dyn Error>> {
    println!("Processing URL: {}", url);
    let html_content = fetch_page_html(client, &url).await?;
    let doc = Document::from(html_content.as_str());
    let body_content: String = doc.find(Name("article"))
                                  .next()
                                  .or_else(|| doc.find(Name("main")).next())
                                  .or_else(|| doc.find(Name("body")).next())
                                  .map_or_else(|| html_content.clone(), |el| el.html());

    let markdown_content = html2md::parse_html(&body_content);
    let slug = url.trim_end_matches('/')
                  .rsplit('/')
                  .next()
                  .map(|s| slugify(s))
                  .unwrap_or_else(|| "index".to_string());
    println!("Saving content to raw/{}.md", &slug);
    save_markdown(&markdown_content, &slug)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "concat" {
        concat_md::main()?;
    } else {
        let client = Client::new();
        let sitemap_url = "https://ispolitical.com/post-sitemap.xml";
        println!("Fetching sitemap...");
        let sitemap_xml = fetch_sitemap_xml(&client, sitemap_url).await?;
        println!("Parsing sitemap...");
        let urls = parse_sitemap(&sitemap_xml)?;

        let concurrent_limit = 10; // Adjust the number of concurrent tasks as needed
        let mut tasks = FuturesUnordered::new();

        for url in urls {
            tasks.push(process_url(&client, url.clone()));

            if tasks.len() >= concurrent_limit {
                if let Some(result) = tasks.next().await {
                    result?;
                }
            }
        }

        while let Some(result) = tasks.next().await {
            result?;
        }

        println!("All done!");
    }

    Ok(())
}
