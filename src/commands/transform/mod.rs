use crate::core::{
    db,
    types::{Command, HelpInfo},
};
use std::{error::Error, path::Path, time::Instant};

mod transform_stats;
use transform_stats::{ProcessingStats, print_stats};

mod record_handler;
use record_handler::{get_record_iterator, process_record};

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
        let sql =
            "INSERT INTO metrics (id, timestamp, value, tag, positive) VALUES (?1, ?2, ?3, ?4, ?5)";
        let mut stmt = tx.prepare(sql)?;

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
                    if process_record(&mut stmt, record).is_ok() {
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
        drop(stmt);
        tx.commit()?;

        println!("\rDone!\n");

        // Report Metrics
        print_stats(stats, start_time);

        Ok(())
    }

    fn info(&self) -> HelpInfo {
        HelpInfo {
            label: "transform",
            aliases: &["t"],
            description: "Process records, applied a basic transformation and outputs int into 'storage.db'",
            usage: "transform [format]",
        }
    }
}
