use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use crate::models::Record;

pub fn write_ndjson(records: &[Record], path: &str) -> Result<(), Box<dyn Error>> {
    println!("Writing NDJSON to {}...", path);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for record in records {
        // Serialize to JSON and append a newline manually
        serde_json::to_writer(&mut writer, record)?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}
