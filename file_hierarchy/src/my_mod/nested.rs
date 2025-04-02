#![allow(unused)]

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
