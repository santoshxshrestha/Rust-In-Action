use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("./food-price-index-september-2023-seasonally-adjusted.csv");
    
    let file = File::open(file_path)?;
    
    let mut reader = csv::Reader::from_reader(file);
    
    for result in reader.records() {
        let record = result?;
        
        println!("Record: {:?}", record);
        
        if record.len() > 0 {
            println!("First field: {}", &record[0]);
        }
        if record.len() > 1 {
            println!("Second field: {}", &record[1]);
        }
    }
    
    Ok(())
}
