pub mod gen_csv;
pub mod gen_ndjson;

use chrono::{Duration, Utc};
use rand::Rng;
use std::error::Error;
use std::fs::{self};
use std::path::Path;
use uuid::Uuid;

use crate::models::Record;

pub fn generate() -> Result<(), Box<dyn Error>> {
    // 1. Configuration
    let record_count = 100_005;
    let output_dir = "./data";
    let csv_path = format!("{}/input.csv", output_dir);
    let ndjson_path = format!("{}/input.ndjson", output_dir);

    // Ensure output directory
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // 3. Generate Data (Once, so both files match exactly)
    let data = generate_dummy_records(record_count);

    // 4. Write Files
    self::gen_csv::write_csv(&data, &csv_path)?;
    self::gen_ndjson::write_ndjson(&data, &ndjson_path)?;

    println!("Done! Files saved in {}", output_dir);
    Ok(())
}

fn generate_dummy_records(count: usize) -> Vec<Record> {
    // ---------------------------------------------
    let mut generator = rand::rng();
    let tags = vec!["1_tag", "2_tag", "3_tag", "4_tag"];
    let mut records = Vec::with_capacity(count);
    let mut current_time = Utc::now();

    println!("Generating {} records in memory...", count);

    for _ in 0..count {
        current_time = current_time - Duration::seconds(generator.random_range(1..60));

        let raw_value: f64 = generator.random_range(10.0..500.0);
        let value = (raw_value * 100.0).round() / 100.0;

        records.push(Record {
            id: Uuid::new_v4().to_string(),
            timestamp: current_time.to_rfc3339(),
            value,
            tag: tags[generator.random_range(0..tags.len())].to_string(),
        });
    }

    records
}
