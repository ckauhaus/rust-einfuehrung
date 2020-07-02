use futures::future::join_all;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use tokio::runtime::Runtime;

// What we are looking for
static PERSONS: [&str; 2] = ["Merkel", "Trump"];

#[derive(Deserialize, Debug)]
struct Newssite {
    name: String,
    url: String,
}

#[derive(Debug)]
struct Occurences {
    name: String,
    // Counts indexed by person
    counts: Result<Vec<usize>, Box<dyn Error>>,
}

impl fmt::Display for Occurences {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:12}", self.name)?;
        match &self.counts {
            Ok(counts) => Ok(for (n, name) in PERSONS.iter().enumerate() {
                write!(f, " {:30}", name[0..1].repeat(counts[n]))?;
            }),
            Err(e) => write!(f, " Error: {}", e),
        }
    }
}

impl Newssite {
    async fn count_persons(url: &str) -> Result<Vec<usize>, Box<dyn Error>> {
        eprintln!("{}: downloading", url);
        let body = reqwest::get(url).await?.error_for_status()?.text().await?;
        Ok(PERSONS.iter().map(|p| body.matches(p).count()).collect())
    }

    async fn check(self) -> Occurences {
        Occurences {
            name: self.name,
            counts: Self::count_persons(&self.url).await,
        }
    }
}

fn run(sites: Vec<Newssite>) -> Vec<Occurences> {
    let tasks: Vec<_> = sites.into_iter().map(|site| site.check()).collect();
    Runtime::new().unwrap().block_on(join_all(tasks))
}

fn main() -> Result<(), Box<dyn Error>> {
    let sites: Vec<Newssite> = serde_json::from_reader(File::open("newssites.json")?)?;
    for occ in run(sites) {
        println!("{}", occ);
    }
    Ok(())
}
