macro_rules! hello{
    () => {
        println!("Hello, Santosh!");
        
    };
}

macro_rules! fn_creator {
    () => {
        println!("No argument provided");

    };
    ($name_of_fn:ident) => {
        println!("You call the function {:?}.",stringify!($name_of_fn));
        
    };
    (&name_of_fn:expr) => {
        println!("The name of the function is {:?}",$name_of_fn);

    };
}

macro_rules! gap {
    () => {
        println!();
        
    };
}
fn main() {
    hello!();
    gap!();

    fn_creator!();
    gap!();

    fn_creator!(santosh);
    gap!();

    fn_creator!(another_name);
    gap!();

}
