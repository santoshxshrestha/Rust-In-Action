#![allow(unused_imports)]
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    // here though we can change the pointer of the Rc but we cannot change the content of the Rc
    {
        let mut x = Rc::new(String::from("here are some string that might be mutable "));
        println!("the content of the rc is :{x}");
        x = Rc::new(String::from("I want to change the content again "));
        println!("the content of the rc is :{x}");
    }
    // for changing the content of the rc we have to use the Rc<RefCell<T>
    {
        // here this is not need to be mutable because we are not changing the pointer of the Rc
        let rc = Rc::new(RefCell::new(String::from(
            "here are some content in the refcell",
        )));

        // we need to borrow the content of the refcell to change it
        println!("the content of the rc with refcell is {}", rc.borrow());
        rc.borrow_mut().push_str(" and I am changing it");
        println!("the content of the rc with refcell is {}", rc.borrow());
    }
}
