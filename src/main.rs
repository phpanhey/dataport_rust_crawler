use reqwest::Error;
use select::document::Document;
use select::predicate::Name;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _pattern = &args[1];
    let urls = &args[2..args.len()];

    let hrefs = job_urls(urls);
    println!("{:?}",hrefs);
}

pub fn get_html(url: &str) -> Result<String, Error> {
    return reqwest::blocking::get(url)?.text();
}

pub fn job_urls(urls: &[String]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for url in urls {
        let html = get_html(url);
        let document = Document::from(html.unwrap().as_str());
        for item in document.find(Name("a")) {
            let elem = item.attr("href");
            if elem != None && is_candidate(elem) {
                res.push(elem.unwrap().to_string());
            }
        }
    }
    return res;
}

pub fn is_candidate(url_option: Option<&str>) -> bool {
    let url = url_option.unwrap();
    return url.contains("karriere.")
        && url.split("/").collect::<Vec<&str>>()[3]
            .chars()
            .next()
            .unwrap()
            .is_uppercase();
}
