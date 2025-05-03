#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn example_automatic () {
    let p1 = Point {
        x: 1,
        y: 32
    };
    let p2 = Point{
        x: 1,
        y: 32,
    };
    assert_eq!(p1, p2);

    use std::collections::HashSet;
    let mut points = HashSet::new();
    points.insert(p1);
    assert!(points.contains(&p2));

}

#[derive(Debug)]
struct  Person {
    id: u32,
    name: String,
    email: String,
}

impl PartialEq for Person{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}

impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn example_manual () {
    let p1 = Person{
        id: 1,
        name: "Santosh".to_owned(),
        email: "santosh@gmail.com".to_owned(),
    };
    let p2 = Person{
        id : 1,
        name: "Santosh Shrestha".to_owned(),
        email: "santosh.shrestha@gmail.com".to_owned(),
    };
    //here we are not getting error because in the impl of the eq trait above we are 
    //specifing that we are comparing the id's of the items of the sturctures
    assert_eq!(p1, p2);


    //using the hash map 
    let mut people = HashMap::new();
    people.insert(p1, "Employee");

    people.insert(p2, "Manager");
    //here wer are not getting the error because the equality is based on the ID 
    //and we have both the ID's same out there in the structure
    assert_eq!(people.len(), 1);

}

//not a way that you should always do but yah it is what it is 
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}
 impl PartialEq for Vector3D {
     fn eq(&self, other: &Self) -> bool {
         (self.x - other.x).abs() < 1e-10 && 
         (self.y - other.y).abs() < 1e-10 && 
         (self.z - other.z).abs() < 1e-10 
     }
     
 }

impl std::hash::Hash for Vector3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();
        let z_bits = self.z.to_bits();

        x_bits.hash(state);
        y_bits.hash(state);
        z_bits.hash(state);
    }
    
}


#[derive(Debug, Clone)]
struct CaseInsensetiveString(String);

impl PartialEq for CaseInsensetiveString{
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }

} 

impl Eq for CaseInsensetiveString {

}

impl std::hash::Hash for CaseInsensetiveString{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}

fn example_case_insensative() {

    let s1 = CaseInsensetiveString("Hello".to_string());
    let s2 = CaseInsensetiveString("HELLO".to_string());
    let s3 = CaseInsensetiveString("hello".to_string());
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    assert_eq!(s1, s3);
}

#[derive(Debug)]
struct Book {
    isbn: String, 
    title: String,
    author: String,
    publication_year: u32,
    price: f64,
}

impl PartialEq for Book{
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

impl Eq for Book{
    
}

impl std::hash::Hash for Book {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.isbn.hash(state);
    }
}




fn main() {
    example_automatic();
    example_manual ();
    example_case_insensative();


    //example with book
    let book1 = Book{
        isbn: "978-452525252".to_string(),
        title: "Programming Rust, 2nd Edition".to_string(),
        author: "Jim Blandy and Jason Orendorff".to_string(),
        publication_year: 2017,
        price: 99.99,
    };

    let book2 = Book{
        isbn: "978-452525252".to_string(),
        title: "Programming Rust".to_string(),
        author: "Jim Blandy".to_string(),
        publication_year: 2017,
        price: 99.99,
    };

    //we are just checking the isbn number so it will work fine because we got the isbn number same 
    //for the both books out there.
    assert_eq!(book1, book2);

    {


        {
            let mut hasher = DefaultHasher::new(); 
            123.hash(&mut hasher);                
            "hello".hash(&mut hasher);           
            let result = hasher.finish();       
            println!("Hash value: {}", result);
        }
        println!();

        {
            let mut hasher = DefaultHasher::new();
            "santosh".hash(&mut hasher);
            let result = hasher.finish();
            println!("Hash value is : {}", result);

            "another santosh".hash(&mut hasher);
            let anotehr_result = hasher.finish();
            println!("This is the another hash val : {}", anotehr_result);
        }

        println!();
        {
            let mut hasher = DefaultHasher::new();
            "santosh".hash(&mut hasher);
            let result = hasher.finish();
            println!(r#"The hash val of the "santosh"is {}"#,result);


            let same_result = hasher.finish();
            println!("The hash val of the same hasher is {}",same_result);
        }
        
        println!();
        {
            let mut hasher = DefaultHasher::new() ;
            "santosh".hash(&mut hasher);
            let result = hasher.finish();
            println!(r#"The hash val of the "santosh"{}"#,result); 

            "santosh".hash(&mut hasher);
            let another_result = hasher.finish();
            //I will be getting completely difference hash val  if I introduced some thing 
            //because the val adds up and generate new hash val out there and the .finish will 
            //ge the val at that state 
            println!("This it the second iteration of the hasher {}",another_result);
        }
        println!();
    }

}
