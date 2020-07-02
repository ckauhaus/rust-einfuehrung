use futures::future::join_all;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use tokio::runtime::Runtime;

// What we are looking for
static PERSONS: &[&str] = &["Merkel", "Trump"];

#[derive(Deserialize, Debug)]
struct Newssite {
    name: String,
    url: String,
}

#[derive(Debug)]
struct Occurences {
    site: Newssite,
    // Counts indexed by person
    counts: Result<Vec<usize>, reqwest::Error>,
}

impl Occurences {
    fn print(&self) {
        match &self.counts {
            Ok(counts) => {
                print!("{:12}", self.site.name);
                for (n, name) in PERSONS.iter().enumerate() {
                    print!(" {:30}", name[0..1].repeat(counts[n]))
                }
                println!()
            }
            Err(e) => println!("{:12} Error: {}", self.site.name, e),
        }
    }
}

async fn count_persons(url: &str) -> Result<Vec<usize>, reqwest::Error> {
    eprintln!("{}: downloading", url);
    let body = reqwest::get(url).await?.error_for_status()?.text().await?;
    Ok(PERSONS.iter().map(|p| body.matches(p).count()).collect())
}

impl Newssite {
    async fn check(self) -> Occurences {
        let counts = count_persons(&self.url).await;
        Occurences { site: self, counts }
    }
}

fn run(sites: Vec<Newssite>) -> Vec<Occurences> {
    let tasks: Vec<_> = sites.into_iter().map(|site| site.check()).collect();
    Runtime::new().unwrap().block_on(join_all(tasks))
}

fn main() -> Result<(), Box<dyn Error>> {
    let sites: Vec<Newssite> = serde_json::from_reader(File::open("newssites.json")?)?;
    for occ in run(sites) {
        occ.print();
    }
    Ok(())
}
