extern crate reqwest;
extern crate select;
extern crate html2md;
extern crate serde;

use std::fs;
use std::error::Error;
use std::path::Path;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use serde::Deserialize;
use serde_xml_rs::from_str;
use slug::slugify;

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

fn fetch_sitemap_xml(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(response)
}

fn parse_sitemap(xml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let urlset: Urlset = from_str(xml)?;
    Ok(urlset.urls.into_iter().map(|url| url.loc).collect())
}

fn fetch_page_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(response)
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

fn main() -> Result<(), Box<dyn Error>> {
    let sitemap_url = "https://ispolitical.com/post-sitemap.xml";
    println!("Fetching sitemap...");
    let sitemap_xml = fetch_sitemap_xml(sitemap_url)?;
    println!("Parsing sitemap...");
    let urls = parse_sitemap(&sitemap_xml)?;

    for url in urls {
        println!("Processing URL: {}", url);
        let html_content = fetch_page_html(&url)?;
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
        println!("Saving content to raw/{}.md", slug);
        save_markdown(&markdown_content, &slug)?;
    }

    println!("All done!");
    Ok(())
}
