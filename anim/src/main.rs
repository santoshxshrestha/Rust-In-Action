use indicatif::{ProgressBar, ProgressStyle};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn scan_port(port: usize) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100)).is_ok()
}

pub fn get_port() -> Vec<usize> {
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    let chunk_size = 100;
    let total_ports = 9999; // 1 to 9999
    let total_chunks = (total_ports as f64 / chunk_size as f64).ceil() as u64;

    // Create progress bar
    let pb = ProgressBar::new(total_chunks);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {pos}/{len} chunks completed - Scanning ports...")
            .unwrap(),
    );

    for chunk_start in (1..10000).step_by(chunk_size) {
        let chunk_end = std::cmp::min(chunk_start + chunk_size, 10000);
        let open_ports_clone = Arc::clone(&open_ports);
        let pb_clone = pb.clone();

        let handle = thread::spawn(move || {
            let mut local_open_ports = Vec::new();
            for port in chunk_start..chunk_end {
                if scan_port(port) {
                    local_open_ports.push(port);
                    println!("Port {} is open", port);
                }
            }
            // Adding found ports to the shared vector
            if !local_open_ports.is_empty() {
                let mut ports = open_ports_clone.lock().unwrap();
                ports.extend(local_open_ports);
            }
            // Update progress bar
            pb_clone.inc(1);
        });
        handles.push(handle);
    }

    // Waiting for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    pb.finish_with_message("Port scan completed!");

    //The final list of open ports
    let final_open_ports = {
        let ports = open_ports.lock().unwrap();
        ports.clone()
    };

    final_open_ports
}

fn main() {
    println!("Starting port scan on 127.0.0.1...");
    let open_ports = get_port();
    println!(
        "Scan finished! Found {} open ports: {:?}",
        open_ports.len(),
        open_ports
    );
}
