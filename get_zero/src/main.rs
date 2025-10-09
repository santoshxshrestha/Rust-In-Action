fn main() {
    let array = [1, 2, 1, 2, 12, 12, 12, 12, 2, 41];
    /// this will return the first elemetn of the array if it is out of bounds it will return None
    if let Some(value) = array.get(0) {
        println!("{}", value);
    }
}
