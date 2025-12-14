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

pub struct TransformCommand;

impl Command for TransformCommand {
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn Error>> {
        let format = args.get(1).copied().unwrap_or("csv");
        let filename = format!("input.{}", format);

        let input_path = Path::new("data").join(filename);
        let db_path = "storage.db";
        let mut connection = db::setup_db(db_path)?;

        let start_time = Instant::now();

        println!("Processing {}...", input_path.display());

        // Determine file type and get an Iterator of Results
        // We box the iterator so we can treat CSV and JSON the same in the loop
        let extension = Path::new(&input_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        // This creates a generic iterator that yields Records
        // We use Box<dyn Iterator...> to handle the different types (CSV vs JSON) dynamically
        let record_iterator: Box<dyn Iterator<Item = Result<Record, Box<dyn Error>>>> =
            match extension {
                "csv" => {
                    let reader = csv::ReaderBuilder::new()
                        .has_headers(true)
                        .from_path(input_path)?;

                    Box::new(reader.into_deserialize().map(|r| r.map_err(|e| e.into())))
                }
                "ndjson" => {
                    let file = File::open(input_path)?;
                    let reader = BufReader::new(file);

                    // Map JSON lines to Result
                    Box::new(reader.lines().map(|line| {
                        let line_str = line?;
                        let record: Record = serde_json::from_str(&line_str)?;
                        Ok(record)
                    }))
                }
                _ => {
                    let err = io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unsupported file extension: {}", extension),
                    );
                    return Err(Box::new(err));
                }
            };

        // Start a Transaction (Batching)
        // Writing 100k rows individually is slow. A transaction groups them.
        let tx = connection.transaction()?;

        let mut total_processed = 0;
        let mut success_count = 0;
        let mut fail_count = 0;

        // 3. Stream and Transform
        for result in record_iterator {
            total_processed += 1;

            match result {
                Ok(record) => {
                    // --- Transformation Logic ---

                    let tag_normalized = record.tag.unwrap_or_default().trim().to_lowercase();

                    // Skip empty tags
                    if tag_normalized.is_empty() {
                        fail_count += 1; // Or just skip without counting as fail, depending on requirements
                        continue;
                    }

                    // Parse Timestamp (ISO-8601 -> Unix Timestamp)
                    // We use DateTime::parse_from_rfc3339 for ISO format
                    let parsed_timestamp = match DateTime::parse_from_rfc3339(&record.timestamp) {
                        Ok(dt) => dt.timestamp(),
                        Err(_) => {
                            fail_count += 1;
                            continue;
                        }
                    };

                    // 4. Derive Positive Field
                    let positive = if record.value > 0.0 { 1 } else { 0 };

                    // --- Database Write ---
                    // We pass &tx because Transaction implements Deref<Target=Connection>
                    match db::insert_record(
                        &tx,
                        &record.id,
                        parsed_timestamp,
                        record.value,
                        &tag_normalized,
                        positive,
                    ) {
                        Ok(_) => success_count += 1,
                        Err(_) => fail_count += 1,
                    }
                }
                Err(_) => {
                    // Parse error (malformed CSV/JSON)
                    fail_count += 1;
                }
            }

            // Optional: Print progress every 10k rows
            if total_processed % 10_000 == 0 {
                print!("\rProcessed: {}", total_processed);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }

        // 4. Commit the Transaction
        tx.commit()?;

        println!("\rDone!              ");

        // 5. Report Metrics
        let duration = start_time.elapsed();
        let seconds = duration.as_secs_f64();
        let rows_per_sec = if seconds > 0.0 {
            success_count as f64 / seconds
        } else {
            0.0
        };

        println!("---------------------------------");
        println!("Processing Metrics:");
        println!("---------------------------------");
        println!("Total records processed : {}", total_processed);
        println!("Successful rows written : {}", success_count);
        println!("Failed rows (skipped)   : {}", fail_count);
        println!("Total duration          : {:.2?}", duration);
        println!("Throughput              : {:.0} rows/sec", rows_per_sec);
        println!("---------------------------------");

        Ok(())
    }

    fn info(&self) -> crate::models::HelpInfo {
        HelpInfo {
            name: "transform",
            aliases: &["t"],
            description: "Process records, applied a basic transformation and outputs int into 'storage.db'",
            usage: "transform [format]",
        }
    }
}

// pub fn transform(filename: &str) -> Result<(), Box<dyn Error>> {
//     let input_path = Path::new("data").join(filename);
//     let db_path = "storage.db";
//     let mut connection = db::setup_db(db_path)?;

//     let start_time = Instant::now();

//     println!("Processing {}...", input_path.display());

//     // Determine file type and get an Iterator of Results
//     // We box the iterator so we can treat CSV and JSON the same in the loop
//     let extension = Path::new(&input_path)
//         .extension()
//         .and_then(|ext| ext.to_str())
//         .unwrap_or("");

//     // This creates a generic iterator that yields Records
//     // We use Box<dyn Iterator...> to handle the different types (CSV vs JSON) dynamically
//     let record_iterator: Box<dyn Iterator<Item = Result<Record, Box<dyn Error>>>> = match extension
//     {
//         "csv" => {
//             let reader = csv::ReaderBuilder::new()
//                 .has_headers(true)
//                 .from_path(input_path)?;

//             Box::new(reader.into_deserialize().map(|r| r.map_err(|e| e.into())))
//         }
//         "ndjson" | "jsonl" => {
//             let file = File::open(input_path)?;
//             let reader = BufReader::new(file);

//             // Map JSON lines to Result
//             Box::new(reader.lines().map(|line| {
//                 let line_str = line?;
//                 let record: Record = serde_json::from_str(&line_str)?;
//                 Ok(record)
//             }))
//         }
//         _ => {
//             let err = io::Error::new(
//                 io::ErrorKind::InvalidInput,
//                 format!("Unsupported file extension: {}", extension),
//             );
//             return Err(Box::new(err));
//         }
//     };

//     // Start a Transaction (Batching)
//     // Writing 100k rows individually is slow. A transaction groups them.
//     let tx = connection.transaction()?;

//     let mut total_processed = 0;
//     let mut success_count = 0;
//     let mut fail_count = 0;

//     // 3. Stream and Transform
//     for result in record_iterator {
//         total_processed += 1;

//         match result {
//             Ok(record) => {
//                 // --- Transformation Logic ---

//                 let tag_normalized = record.tag.trim().to_lowercase();

//                 // Skip empty tags
//                 if tag_normalized.is_empty() {
//                     fail_count += 1; // Or just skip without counting as fail, depending on requirements
//                     continue;
//                 }

//                 // Parse Timestamp (ISO-8601 -> Unix Timestamp)
//                 // We use DateTime::parse_from_rfc3339 for ISO format
//                 let parsed_timestamp = match DateTime::parse_from_rfc3339(&record.timestamp) {
//                     Ok(dt) => dt.timestamp(),
//                     Err(_) => {
//                         fail_count += 1;
//                         continue;
//                     }
//                 };

//                 // 4. Derive Positive Field
//                 let positive = if record.value > 0.0 { 1 } else { 0 };

//                 // --- Database Write ---
//                 // We pass &tx because Transaction implements Deref<Target=Connection>
//                 match db::insert_record(
//                     &tx,
//                     &record.id,
//                     parsed_timestamp,
//                     record.value,
//                     &tag_normalized,
//                     positive,
//                 ) {
//                     Ok(_) => success_count += 1,
//                     Err(_) => fail_count += 1,
//                 }
//             }
//             Err(_) => {
//                 // Parse error (malformed CSV/JSON)
//                 fail_count += 1;
//             }
//         }

//         // Optional: Print progress every 10k rows
//         if total_processed % 10_000 == 0 {
//             print!("\rProcessed: {}", total_processed);
//             use std::io::Write;
//             std::io::stdout().flush().unwrap();
//         }
//     }

//     // 4. Commit the Transaction
//     tx.commit()?;

//     println!("\rDone!              ");

//     // 5. Report Metrics
//     let duration = start_time.elapsed();
//     let seconds = duration.as_secs_f64();
//     let rows_per_sec = if seconds > 0.0 {
//         success_count as f64 / seconds
//     } else {
//         0.0
//     };

//     println!("---------------------------------");
//     println!("Processing Metrics:");
//     println!("---------------------------------");
//     println!("Total records processed : {}", total_processed);
//     println!("Successful rows written : {}", success_count);
//     println!("Failed rows (skipped)   : {}", fail_count);
//     println!("Total duration          : {:.2?}", duration);
//     println!("Throughput              : {:.0} rows/sec", rows_per_sec);
//     println!("---------------------------------");

//     Ok(())
// }
