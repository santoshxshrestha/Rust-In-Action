use local_ip_address::local_ip;
fn main() {
    let local_ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to get local IP address: {}", e);
            return;
        }
    };

    println!("Local IP address: {}", local_ip);
}
