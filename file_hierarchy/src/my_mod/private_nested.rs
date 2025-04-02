
        #![allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
