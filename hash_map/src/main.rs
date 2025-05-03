
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



fn main() {
    example_automatic();
    example_manual ();

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
