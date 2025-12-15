use crate::core::{db, types::Record};
use chrono::DateTime;
use rusqlite::Statement;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn get_record_iterator(
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
                .from_path(input_path)
                .map_err(|e| {
                    format!(
                        "\x1b[31m\nFailed to open CSV file: {:?}:\n{}\nRun `generate` first.\x1b[0m",
                        input_path, e
                    )
                })?;
            Ok(Box::new(
                reader.into_deserialize().map(|r| r.map_err(|e| e.into())),
            ))
        }
        "ndjson" => {
            let file = File::open(input_path).map_err(|e| {
                format!(
                    "\x1b[31m\nFailed to open NDJSON file: {:?}:\n{}\nRun `generate ndjson` first.\x1b[0m",
                    input_path, e
                )
            })?;

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
pub fn process_record(stmt: &mut Statement, record: Record) -> Result<(), ()> {
    // let tag_raw = record.tag.as_deref().unwrap_or("");
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

    match stmt.execute(rusqlite::params![
        record.id,
        parsed_timestamp,
        record.value,
        tag_normalized,
        positive,
    ]) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }

    // match db::insert_record(
    //     tx,
    //     &record.id,
    //     parsed_timestamp,
    //     record.value,
    //     &tag_normalized,
    //     positive,
    // ) {
    //     Ok(_) => Ok(()),
    //     Err(_) => Err(()),
    // }
}
