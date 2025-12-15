use std::{
    error::Error,
    fs::{self},
    path::Path,
};

use chrono::{Duration, Utc};
use dotenvy::dotenv;
use rand::Rng;
use std::env;
use uuid::Uuid;

pub mod gen_csv;
pub mod gen_ndjson;

use crate::core::types::{Command, HelpInfo, Record};

pub struct GenerateCommand;
impl Command for GenerateCommand {
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn Error>> {
        dotenv().ok();

        let format = args.get(0).copied().unwrap_or("csv");
        // let record_count = 100_000;
        let record_count = env::var("RECORD_COUNT")
            .unwrap_or_else(|_| "100000".to_string())
            .parse()
            .expect("RECORD_COUNT must be a valid number");
        let output_dir = "./data";
        let csv_path = format!("{}/input.csv", output_dir);
        let ndjson_path = format!("{}/input.ndjson", output_dir);

        if !Path::new(output_dir).exists() {
            fs::create_dir_all(output_dir)?;
        }

        // Generate Data
        let data = generate_dummy_records(record_count);

        match format {
            "csv" => {
                self::gen_csv::write_csv(&data, &csv_path)?;
            }

            // You can stack patterns using `|` (OR)
            "ndjson" | "json" => {
                self::gen_ndjson::write_ndjson(&data, &ndjson_path)?;
            }

            // Good practice to have an "all" option
            "all" => {
                println!("Writing all formats...");
                self::gen_csv::write_csv(&data, &csv_path)?;
                self::gen_ndjson::write_ndjson(&data, &ndjson_path)?;
            }

            // Handle unknown formats safely
            unknown => {
                // .into() converts the String message into a Box<dyn Error>
                return Err(format!(
                    "Unknown format: '{}'. Use 'csv', 'ndjson', or 'all'.",
                    unknown
                )
                .into());
            }
        }
        Ok(())
    }

    fn info(&self) -> HelpInfo {
        HelpInfo {
            label: "generate",
            aliases: &["g", "gen"],
            description: "Generates dummy data (CSV/NDJSON) for the project.",
            usage: "generate [format]",
        }
    }
}

fn generate_dummy_records(count: usize) -> Vec<Record> {
    let mut generator = rand::rng();
    let tags: Vec<Option<&str>> = vec![
        Some("1_tag"),
        Some("2_tag"),
        Some("3_tag"),
        Some("4_tag"),
        Some(" padded_tag "),
        Some("Mixed_Tag"),
        Some("UPPERCASE_TAG"),
        None, // add some 'null' tags
    ];
    let mut records = Vec::with_capacity(count);
    let mut current_time = Utc::now();

    println!("Generating {} records in memory...", count);

    for i in 0..count {
        if i > 0 && i % 100_000 == 0 {
            println!("  Processed {} records...", i);
        }

        current_time = current_time - Duration::seconds(generator.random_range(1..60));

        let raw_value: f64 = generator.random_range(-50.0..500.0);
        let value = (raw_value * 100.0).round() / 100.0;

        records.push(Record {
            id: Uuid::new_v4().to_string(),
            timestamp: current_time.to_rfc3339(),
            value,
            tag: tags[generator.random_range(0..tags.len())].map(|s| s.to_string()),
        });
    }

    records
}
