use crate::{
    db,
    models::{Command, HelpInfo, Record},
};
use chrono::DateTime;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    time::Instant,
};

#[derive(Default)] // Optional: allows ProcessingStats::default()
struct ProcessingStats {
    total: u64,
    success: u64,
    fail: u64,
}

pub struct TransformCommand;

impl Command for TransformCommand {
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn Error>> {
        let format = args.get(0).copied().unwrap_or("csv");
        let filename = format!("input.{}", format);
        let input_path = Path::new("data").join(filename);
        let db_path = "storage.db";
        let mut connection = db::setup_db(db_path)?;

        let start_time = Instant::now();

        println!("Processing {}...", input_path.display());

        let record_iterator = get_record_iterator(&input_path)?;

        // Start a Transaction (Batching)
        // Writing 100k rows individually is slow. A transaction groups them.
        let tx = connection.transaction()?;

        let mut stats = ProcessingStats {
            total: 0,
            success: 0,
            fail: 0,
        };

        // Stream and Transform
        for result in record_iterator {
            stats.total += 1;

            match result {
                Ok(record) => {
                    // Transform Record
                    if process_record(&tx, record).is_ok() {
                        stats.success += 1; // success
                    } else {
                        stats.fail += 1; // fail
                    }
                }
                Err(_) => {
                    // Parse error (malformed CSV/JSON)
                    stats.fail += 1;
                }
            }

            // Optional: Print progress every 10k rows
            if stats.total % 10_000 == 0 {
                print!("\rProcessed: {}", stats.total);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }

        // Commit the Transaction
        tx.commit()?;

        println!("\rDone!              ");

        // Report Metrics
        print_metrics(stats, start_time);

        Ok(())
    }

    fn info(&self) -> crate::models::HelpInfo {
        HelpInfo {
            label: "transform",
            aliases: &["t"],
            description: "Process records, applied a basic transformation and outputs int into 'storage.db'",
            usage: "transform [format]",
        }
    }
}

fn get_record_iterator(
    input_path: &Path,
) -> Result<Box<dyn Iterator<Item = Result<Record, Box<dyn Error>>>>, Box<dyn Error>> {
    let extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "csv" => {
            let reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_path(input_path)?;
            Ok(Box::new(
                reader.into_deserialize().map(|r| r.map_err(|e| e.into())),
            ))
        }
        "ndjson" => {
            let file = File::open(input_path)?;
            let reader = BufReader::new(file);
            Ok(Box::new(reader.lines().map(|line| {
                let line_str = line?;
                let record: Record = serde_json::from_str(&line_str)?;
                Ok(record)
            })))
        }
        _ => {
            let err = io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported file extension: {}", extension),
            );
            Err(Box::new(err))
        }
    }
}

// --- Transformation Logic ---
fn process_record(tx: &rusqlite::Transaction, record: Record) -> Result<(), ()> {
    let tag_normalized = record.tag.unwrap_or_default().trim().to_lowercase();

    if tag_normalized.is_empty() {
        return Err(());
    }

    // Parse Timestamp
    let parsed_timestamp = match DateTime::parse_from_rfc3339(&record.timestamp) {
        Ok(dt) => dt.timestamp(),
        Err(_) => return Err(()),
    };

    let positive = if record.value > 0.0 { 1 } else { 0 };

    match db::insert_record(
        tx,
        &record.id,
        parsed_timestamp,
        record.value,
        &tag_normalized,
        positive,
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

fn print_metrics(stats: ProcessingStats, start_time: Instant) {
    let duration = start_time.elapsed();
    let seconds = duration.as_secs_f64();

    let rows_per_sec = if seconds > 0.0 {
        stats.success as f64 / seconds
    } else {
        0.0
    };

    println!("---------------------------------");
    println!("Processing Metrics:");
    println!("---------------------------------");
    println!("Total records processed : {}", stats.total);
    println!("Successful rows written : {}", stats.success);
    println!("Failed rows (skipped)   : {}", stats.fail);
    println!("Total duration          : {:.2?}", duration);
    println!("Throughput              : {:.0} rows/sec", rows_per_sec);
    println!("---------------------------------");
}
