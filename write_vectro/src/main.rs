use std::io::Write;
fn main() {
    let mut vector = Vec::new();
    if let Err(e) = write!(
        &mut vector,
        "this is the first content in the vector : {}",
        "this is the anothe ron ethat is written in teh same line out tehre "
    ) {
        eprintln!(" you got error you dummy {}", e)
    }

    println!(" the contente of the vector is {:?}", vector);
}
