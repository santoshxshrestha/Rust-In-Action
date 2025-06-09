fn main() {
    let addr = "https://192.21.34:23232".to_string();
    let splited: Vec<&str> = addr.split(':').collect();
    let port = splited[2];
    println!("the port is {:?}", port);
}
