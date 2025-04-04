#![allow(dead_code)]

//Basically the traits defines functionality a type must provide
trait Describable {
    //required method (must be implemented by types)
    fn describe(&self) -> String;

    fn default_description(&self) -> String {
        format!("This is a default description")
    }
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}
impl Describable for Book{
    fn describe(&self) -> String {
// Each \" represents an escaped double quote,
// indicating that a literal " should appear in the output string at that position.
        format!("\"{}\"by {}, {} pages", self.title, self.author, self.pages)
    }

    fn default_description(&self) -> String {
        format!("A book titled \"{}\"", self.title)
    }
    // here .to_owned kinda creates a clone so ownership is not passed
    //and the .to_string will also do the same thing 
    //
    //this is what it would look like if it was for default thinge not defining the method 
    // fn default_description(&self) -> String {
    // self.title.to_owned()
    // }
}


struct Car {
    make: String,
    model: String,
    year: u32,
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("{} {} ({})",self.make, self.model, self.year)
            //here we are using the default method by not defining this method 
        
    }
}
//trait bounds 
// Functions can use traits as parameters to accept any type that implements the trait
fn print_description<T: Describable>(item: &T) {
    println!("description: {}", item.describe());
    println!("default: {}",item.default_description());
}

//this is anotehr way using the 'where' clause
fn print_details<T>(item: &T) where
T: Describable,{
    println!("Item detail: {}",item.describe());
}


trait Displayable {
    fn display(&self);
}

impl Displayable for Book {
    fn display(&self) {
        println!(" {}",self.describe());
    }
    
}
//multiple trait bounds
//
fn show_and_describe<T>(item: &T)
    where
        T: Describable + Displayable,
{
    item.display();
    println!("Descirption: {}",item.describe());
}

//trait objects
 fn print_all_description(items: &[&dyn Describable]){
     for item in items {
         println!("{}",item.describe());
     }
 }

//generic traits with asociated types 
trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
 }

struct VecContainer<T> {
    items: Vec<T>,
}
impl<T> VecContainer<T> {
    fn new() -> Self{
        VecContainer { items: Vec::new() }
    }
}

//implementing the container for VecContainer 
impl<T> Container for VecContainer<T>{
    type Item = T;

    fn add(&mut self, item: Self::Item){
        self.items.push(item);
    }
    fn get(&self, index: usize) -> Option<&Self::Item> {
       self.items.get(index)
    }
}

trait GenericContainer<T> {
    fn add_item(&mut self, item: T);
    fn get_item(&self, index: usize) -> Option<&T>;
}


//trait inherit form other traits
trait AdvanceDescribable: Describable {
    fn detailed_description(&self) -> String;
}

impl AdvanceDescribable for Book {
    fn detailed_description(&self) -> String {
        format!("{} - A {} page book written by {}", 
            self.describe(), self.pages, self.author)
    }
}

fn main() {

    let book = Book{
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
        pages: 320,
    };

    let car = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2022,
    };

    println!("Book: {}",book.describe());
    println!("Car: {}", car.describe());

    println!("Car default: {}",car.default_description());

    print_description(&book);
    print_details(&car);

    //multiple trait bound thinge here 
    show_and_describe(&book);
    show_and_describe(&book);

    let describabbles: Vec<&dyn Describable> = vec![&book, &car];
    print_all_description(&describabbles);


    let mut container = VecContainer::new();
    container.add(String::from("Item 1"));
    container.add(String::from("Item 2"));


    println!("Advance: {}", book.detailed_description());
    println!("container item: {:?}",container.get(0));
}
