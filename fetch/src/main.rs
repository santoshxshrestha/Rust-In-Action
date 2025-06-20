#![allow(unused)]
use reqwest::{self, Client};
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>, // some repos may not have descriptions
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let username = "santoshxshrestha";
    let url = format!(
        "https://api.github.com/users/{}/repos?per_page=100",
        username
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-script")
        .header("Authorization", format!("token {}", token))
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;

    println!("{:?}", response);

    Ok(())
}
