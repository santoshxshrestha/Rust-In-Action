#![allow(unused)]
//life times can't be written with out & sign because it is 
//associated with refrence
fn print_once<'a> (x:&'a mut i32) {
    *x+=1;
    print!("This is the content here {}\n", x);

}
fn main() {

    let mut x = 89;
    print_once(&mut x);

     {

         //explicitly defined lifetimes for its methods (which are redundant in this case because
         //the code base is small)
        {
            struct Owner(i32);
            impl Owner {
                fn add_one<'a>(&'a mut self) { self.0 += 1; }
                fn print<'a>(&'a self) {
                    println!("`print`: {}", self.0);
                }
            }
            let mut owner = Owner(18);

            owner.add_one();
            owner.print();
        }

         //implictly defined lifetimes for its methods 
         //relies on Rust's lifetime elision rules, which automatically infer lifetimes

        {
            struct Borrower(i32) ;
            impl Borrower {
                fn add_one(&mut self) {
                    self.0 += 1;
                }
                fn print(&self) {
                    println!("`print`: {}", self.0);
                }
            }

            let mut borrower = Borrower(19);
            borrower.add_one();
            borrower.print();


        }




    }s
}
