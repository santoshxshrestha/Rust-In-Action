mod my_mod{
    use crate::my_mod;

    fn private_function() {
        println!("called 'my_mod::private_funtion()`");
    }
    pub fn public_function() {
        println!("called `my_mod::public_function()`");
    }

    // so we can access the private_funtion inside the same module
    pub fn indirect_access(){
        print!("called `my_mod::indirect_access()`, that \n> ");
        private_function();
        my_mod::nested::function();
    }

    pub mod nested {
        use crate::my_mod;

        pub fn function(){
            println!("called `my_mod::nested::function`");
        }

        fn private_function(){
            println!("called `my_mod::nested::private_function`");
        }

        // so we can access the private_funtion inside the same module
        pub fn indirect_access(){
            print!("called `my_mod::nested::indirect_access`, that \n> ");
            my_mod::private_function();
            private_function();
            public_function_in_nested();
        }

        //restricting visibility using the pub(in pth),  pub(self) and pub(super)
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod`");
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    mod private_nested{
        #![allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
    pub fn call_public_function_in_my_mod() {
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()`");
    }
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

}
