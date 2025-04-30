#![allow(dead_code)]
fn double_parsed_number(input: &str) -> Result<i32, std::num::ParseIntError>{
    input.parse::<i32>().map(|number| number *2)
}

fn transform_grades() {
    let grades = vec![52,52,52,52,32,52,23,52,52,52];

    let adjusted_grades: Vec<i32> = grades.iter().map(|grade| grade + 5).collect();
    println!("Original grades: {:?}", grades);
    println!("Adjusted grades: {:?}", adjusted_grades);
}

fn process_data(data: Option<String>)-> Option<i32>{
    // This chains multiple transformations:
    // 1. Convert to lowercase if Some
    // 2. Count the characters if Some
    // 3. Double the count if Some
    data.map(|s| s.to_lowercase())
        .map(|s| s.len())
        .map(|len| len as i32 *2)
}

fn calculate_area(dimenstions: &str) -> Result<f64, String>{
    let parts: Vec<&str> = dimenstions.split(",").collect();

    if parts.len()!=2{
        return Err("Invalid format! Use width,height".to_string());
    }

    let width_result = parts[0].parse::<f64>();

    // If width_result is Ok, the .map_err() is skipped, and execution moves on to the .and_then() method, which would process the successful width value.
    let area_result = width_result.map_err(|e| format!("Width parsing error: {}", e))
        .and_then(|width| {
            parts[1].parse::<f64>()
                .map_err(|e|{
                    format!("Height parsing error: {}", e)
                })
                .map(|height| width * height)
        });
    area_result
}


fn main() {
    match double_parsed_number("5"){
        Ok(n) => println!("Success! Double value: {}",n),
        Err(e)=> println!("Error: {}",e),
    }
    match double_parsed_number("not_a_number"){
        Ok(n) => println!("Success! Double value: {}",n),
        Err(e)=> println!("Error: {}",e),
    }

    transform_grades();

    let mut string_data = String::new();
    string_data.push_str("Hello santosh");

    let output = process_data(Some(string_data));
    match output {
        Some(x) => println!("The 2 times of the lenght of string_data is: {}",x),
        None => println!("lol did not got the val here"),
    }

   let dimenstion_of_square :&str = "2,2";
   let area_of_square =  calculate_area(dimenstion_of_square);
   match area_of_square {
       Ok(x)=> println!("The area of the square is {x}"),
       Err(x) => println!("{x}"),
   }

   let dimenstion_of_rectangle: &str = "4,2";
   let area_of_rectangle = calculate_area(dimenstion_of_rectangle);
   match area_of_rectangle {
       Ok(x)=> println!("The area of the reactangle is {x}"),
       Err(x) => println!("{x}"),
   }

   let dimenstion_failure: &str = "a,2";
   let area_of_dimension_failure = calculate_area(dimenstion_failure);
   match area_of_dimension_failure {
       Ok(x)=> println!("The area of the dimenstion_failure is {x}"),
       Err(x) => println!("{x}"),
   }
}
