mod my_mod;
fn function() {
    println!("called `function()`");
}
fn main() {
    //we can't call the private_function because it is private by default 
    // my_mod::private_function();

    // we can call the public_function
    my_mod::public_function();
    my_mod::indirect_access();

    println!();

    //similarly for the nested module
    my_mod::nested::function();
    // my_mod::nested::private_function();
    my_mod::nested::indirect_access();

    println!();
    my_mod::call_public_function_in_my_mod();

    println!();
    my_mod::public_function_in_crate();

    function();

}
