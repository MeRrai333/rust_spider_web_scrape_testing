extern crate spider;

use std::fs;
use spider::website::Website;
use spider::tokio;

fn is_spa(content: &str) -> bool {
    content.contains("<script") || content.contains("fetch") || content.contains("XMLHttpRequest")
}

fn to_file_mark_down(website: &Website) {
    let mut file_content = String::from("");
    for page in website.get_pages().unwrap().iter() {
        let md = html2md::rewrite_html(&page.get_html(), false);
        /* if is_spa(&page.get_html()) {
            file_content += "----- SPA -----\nURL:";
        }
        else{
            file_content += "----- SSR -----\nURL:";
        } */
        file_content += "--------------------\nURL: ";
        file_content += page.get_url();
        file_content += "\n";
        file_content += &md;
        file_content += "\n\n";
    }
    fs::write("./output.txt", file_content).expect("Unable to write file");
}

#[tokio::main]
async fn main() {
    let url = "https://heygoody.com/";
    let mut website = Website::new(&url).with_caching(true)
        .with_limit(50) 
        .build()
        .unwrap();

    website.scrape().await;

    to_file_mark_down(&website);
}