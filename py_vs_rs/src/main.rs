use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn main() -> io::Result<()> {

    let file_path = "big_data.txt";
    let max_threads = num_cpus::get(); // Get optimal number of threads based on CPU cores
    
    println!("Processing with {} threads", max_threads);
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(max_threads)
        .build()
        .unwrap();
    
    let total_sum = Arc::new(Mutex::new(0u64));
    
    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    let reader = BufReader::with_capacity(1024 * 1024, file); // 1MB buffer
    
    println!("Processing file: {} ({} bytes)", file_path, file_size);
    
    // Process the file line by line to avoid loading everything into memory
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    // Process chunks of lines in parallel
    pool.install(|| {
        lines.par_chunks(lines.len() / max_threads.max(1)).for_each(|chunk| {
            let mut chunk_sum = 0u64;
            
            for line in chunk {
                for segment in line.split_whitespace() {
                    let segment_sum: u32 = segment
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .sum();
                    
                    chunk_sum += segment_sum as u64;
                }
            }
            
            let mut total = total_sum.lock().unwrap();
            *total += chunk_sum;
            
            println!("Thread finished processing chunk with sum: {}", chunk_sum);
        });
    });
    
    let final_sum = *total_sum.lock().unwrap();
    println!("Total sum of all digits: {}", final_sum);
    
    Ok(())
}
