use std::{error::Error, fs::File, io::BufWriter};

use crate::core::types::Record;

pub fn write_csv(records: &[Record], path: &str) -> Result<(), Box<dyn Error>> {
    println!("Writing CSV to {}...\n", path);
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(BufWriter::new(file));

    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}
