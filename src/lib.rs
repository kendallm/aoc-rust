use reqwest::blocking::Client;
use reqwest::{cookie::Jar, Url};
use std::error::Error;

pub mod aoc_2021;
pub mod aoc_2022;


pub fn get_inputs(year: u32, day: u32) -> Vec<String> {
    let path = format!("src/aoc_{}/day_{}.txt", year, day);
    let input = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => {
            println!("Downloading input file");
            let resp = get_input_from_site(year, day).expect("unable to get inputs");
            std::fs::write(&path, &resp).expect("unable to save file");
            resp
        }
    };
    input.split('\n').map(String::from).collect()
}

pub fn get_input_from_site(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let session_cookie = std::fs::read_to_string(".session")?;
    let cookie = format!("session={};", session_cookie);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let jar = Jar::default();

    jar.add_cookie_str(&cookie, &url.parse::<Url>()?);
    
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::new(jar))
        .user_agent("github.com/kendallm/aoc-rust by mastodon.social/@kendallm")
        .build()?;

    let res = client.get(url)
        .send()?;
    if !res.status().is_success() {
        return Err(std::fmt::Error)?
    }
    let text = res.text()?;
    Ok(text)
}
