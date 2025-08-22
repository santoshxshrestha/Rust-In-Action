use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

fn main() {
    {
        let val = "this is some content here ";
        let boxed = Box::new(val);
        println!(
            "this is the content in the box I guess: here I guess{}",
            boxed
        );
        // the val variable is still valid here because its ownership has not been moved to the box
        println!("the non boxed content is: {}", val);
    }
    {
        let heap_string = String::from("this is some content in the heap");
        let boxed_string = Box::new(heap_string);
        // the heap_string variable is no longer valid here because its ownership has been moved to the box
        // println!("This is the cotent after creating the box{}", heap_string);
        println!("the boxed string content is: {}", boxed_string);
    }
    // lets practice some of the rcs here
    {
        let rc = Rc::new(String::from("this is some content in the rc"));
        let rc_clone = Rc::clone(&rc);
        let another_clone = Rc::clone(&rc);
        println!("the content of the rc:{}", rc);
        println!("the content of the rc clone:{}", rc_clone);
        println!("the content of another clone:{}", another_clone);
        println!("the reference count is: {}", Rc::strong_count(&rc));
        println!(
            "the refrence count of the rc weak one is: {}",
            Rc::weak_count(&rc)
        );
        let one_more_clone = Rc::clone(&rc);
        println!("the refrence count is: {}", Rc::strong_count(&rc));
        println!(
            "the refrence count of the rc weak one is: {}",
            Rc::weak_count(&rc)
        );
        println!("the content of one more clone:{}", one_more_clone);

        println!(
            "the refrence count of the rc weak now is: {}",
            Rc::weak_count(&rc)
        );
        let weak_pointer = Rc::downgrade(&rc);
        // I have to dref to print the  weak pointer
        // still not sure why I have to dref here though the inner val is not
        // printed
        println!(
            "downgrade this is called weak pointer i dunno {:?}",
            weak_pointer
        );
        // oho got it i can upgrade it but why would i first create weak then to later upgrade it
        // lets first print it out
        // i still have to use debug mode here here to print it because it return some or none
        println!("the weak rc after upgrade is: {:?}", weak_pointer.upgrade());
        // I can use match
        // if i use the match the return type should be the same
        // so i am creting a new rc here if the weak pointer is no
        match weak_pointer.upgrade() {
            Some(message) => message,
            None => Rc::new(String::from("the weak pointer is none")),
        };

        println!("weak count now is: {}", Rc::weak_count(&rc));
    }
    // the weak pointer is useful to avoid circular reference
    // here is the small demo
    println!("-----------------------------------------------------------------------------------");
    {
        let strong = Rc::new(String::from("hello this is string in strong"));
        let weak: Weak<String> = Rc::downgrade(&strong);

        println!("the strong count is: {}", Rc::strong_count(&strong));
        println!("the weak count is: {}", Rc::weak_count(&strong));
        // now lets update the weak pointer
        if let Some(upgraded) = weak.upgrade() {
            println!("the upgraded weak pointer is: {}", upgraded);
            println!(
                "the weak count after upgrade is: {}",
                Rc::weak_count(&strong)
            );
            println!(
                "the strong count after upgrade is: {}",
                Rc::strong_count(&strong)
            );
        } else {
            println!("the weak pointer could not be upgraded");
        }
        // now if i drop the strong pointer
        drop(strong);
        if let Some(upgraded) = weak.upgrade() {
            println!("again got the value {}", upgraded);
        } else {
            println!("the weak pointer could not be upgraded after dropping the strong pointer");
        }
    }
    // here is the reason that we prefer the weak one some time
    // dont worry the compiler will indeed warn you that is the memory leak
    {
        #![allow(unused)]
        struct Node {
            value: i32,
            next: Option<Rc<RefCell<Node>>>,
            prev: Option<Weak<RefCell<Node>>>, // Use Weak here!
        }
        let first = Rc::new(RefCell::new(Node {
            value: 1,
            next: None,
            prev: None,
        }));
        let second = Rc::new(RefCell::new(Node {
            value: 2,
            next: None,
            prev: None,
        }));
        // my computer might freeze if I uncomment the below code
        // // Create cycle: first <-> second
        // first.borrow_mut().next = Some(second.clone());
        // second.borrow_mut().prev = Some(first.clone());
        //
        // // Both nodes will never be dropped due to the cycle!
        // println!(
        //     "first strong = {}, second strong = {}",
        //     Rc::strong_count(&first),
        //     Rc::strong_count(&second)
        // );
    }
    // here below is the correct way to do it with weak
    {
        #[allow(dead_code)]
        struct Node {
            value: i32,
            next: Option<Rc<RefCell<Node>>>,
            prev: Option<Weak<RefCell<Node>>>, // Use Weak here!
        }

        let first = Rc::new(RefCell::new(Node {
            value: 1,
            next: None,
            prev: None,
        }));
        let second = Rc::new(RefCell::new(Node {
            value: 2,
            next: None,
            prev: None,
        }));

        // Link nodes
        first.borrow_mut().next = Some(second.clone());
        second.borrow_mut().prev = Some(Rc::downgrade(&first)); // Use Weak

        // Now, no cycle: when first and second go out of scope, memory is freed!
        println!(
            "first strong = {}, second strong = {}",
            Rc::strong_count(&first),
            Rc::strong_count(&second)
        );
        println!(
            "first weak = {}, second weak = {}",
            Rc::weak_count(&first),
            Rc::weak_count(&second)
        );
    }
}
// ok this session was really helpful to understand the smart pointers in rust
