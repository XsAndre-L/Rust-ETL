use std::time::Instant;

#[derive(Default)] // Optional: allows ProcessingStats::default()
pub struct ProcessingStats {
    pub total: u64,
    pub success: u64,
    pub fail: u64,
}

pub fn print_stats(stats: ProcessingStats, start_time: Instant) {
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
