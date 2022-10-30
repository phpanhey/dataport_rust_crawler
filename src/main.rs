use reqwest::Error;
fn main() {
    let res = get_html("https://www.rust-lang.org");
    println!("{:?}", res);
}

pub fn get_html(url: &str) -> Result<String, Error> {
    return reqwest::blocking::get(url)?.text();
}
