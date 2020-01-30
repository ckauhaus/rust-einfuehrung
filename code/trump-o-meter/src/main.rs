use futures::future;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

static PERSONS: &[&str] = &["Merkel", "Trump"];

// Count indexed by PERSONS
type Occurences = Vec<usize>;

// Site name -> URL
#[derive(Deserialize, Debug)]
struct Newssites(HashMap<String, String>);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("newssites.json")?;
    let sites: Newssites = serde_json::from_str(&json)?;
    drop(json);
    let tasks: Vec<_> = sites.0.iter().map(|(site, url)| print(site, url)).collect();
    future::join_all(tasks).await;
    Ok(())
}

async fn check(url: &str) -> Result<Occurences, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(format!(
            "{}: {}",
            url,
            response
                .status()
                .canonical_reason()
                .unwrap_or("Generic server error")
        )
        .into());
    }
    let body = response.text().await?;
    Ok(PERSONS
        .iter()
        .map(|person| body.matches(person).count())
        .collect())
}

async fn print(site: &str, url: &str) {
    match check(url).await {
        Ok(counts) => {
            print!("{:12}", site);
            for (i, name) in PERSONS.iter().enumerate() {
                print!(" {:30}", name[0..1].repeat(counts[i]))
            }
            println!()
        }
        Err(e) => println!("{:12} Error: {}", site, e),
    }
}
