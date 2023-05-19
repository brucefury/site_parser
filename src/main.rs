use std::env;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::error::Error;
use chrono::prelude::*;

use reqwest::blocking::Client;
use csv::Reader;
use csv::Writer;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    sequences: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Please provide the path to the folder where the CSV and JSON files are located.".into());
    }
    let file_path = &args[1];

    //Read sequences from JSON
    let config_file_path = format!("{}/config.json", file_path);
    let config_file = File::open(config_file_path).expect("file should open read only");
    let reader = BufReader::new(config_file);
    let config: Config = serde_json::from_reader(reader).expect("JSON was not well-formatted");

    //Convert sequences to a HashSet for efficient lookup
    let sequences: HashSet<_> = config.sequences.iter().cloned().collect();

    //Read URLs from CSV
    let csv_file_path = format!("{}/websites.csv", file_path);
    let file = match File::open(&csv_file_path) {
        Err(why) => panic!("couldn't open {}: {}", csv_file_path, why.to_string()),
        Ok(file) => file,
    };

    //Prepare csv
    let now = Utc::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S");

    let output_file_path = format!("{}/output_{}.csv", file_path, timestamp);
    let mut wtr = Writer::from_path(output_file_path)?;
    wtr.write_record(&["Website", "Sequence Found", "Sequences"])?;

    //Bypass crawler blocking where possible
    let client = Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0;Win64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.82 Safari/537.36")
    .build()?;

    let mut rdr = Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let url = &record[0];
        println!("Processing URL: {}", url);

        match client.get(url).send() {
            Ok(resp) => {
                let text = resp.text()?;
                
                let mut found = false;
                let mut found_sequences = vec![];

                for sequence in &sequences {
                    if text.contains(sequence) {
                        println!("Found sequence: {}", sequence);
                        found = true;
                        found_sequences.push(sequence);
                    }
                }

                wtr.write_record(&[url, &found.to_string(), &found_sequences.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("|")])?;
            }
            Err(_) => {
                wtr.write_record(&[url, &"false".to_string(), &"Error sending request.".to_string()])?;
                println!("Failed to send request to {}", url);
            },
        };
    }

    wtr.flush()?;
    Ok(())
}
