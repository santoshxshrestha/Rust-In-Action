#![allow(unused)]
use std::process::Command;

struct WifiNetwork {
    ssid: String,
    security: String,
}

fn scan_networks() {
    let output = Command::new("nmcli")
        .arg("-t")
        .arg("-f")
        .arg("IN-USE,SSID,SECURITY")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()
        .expect("Failed to execute nmcli command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut networks = Vec::new();
    for line in output_str.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 {
            let in_use = parts[0];
            let ssid = parts[1].to_string();
            let security = parts[2].to_string();
            let prefix = if in_use == "*" { "[*] " } else { "[ ] " };
            networks.push(WifiNetwork { ssid, security });
        }
    }
    for network in networks {
        // println!(
        //     "SSID: {}, Signal Strength: {}, Security: {}",
        //     network.ssid, network.signal_strength, network.security
        // );
        println!("{}", network.ssid);
        // println!("{}", network.security)
    }
}

fn main() {
    scan_networks();
}
