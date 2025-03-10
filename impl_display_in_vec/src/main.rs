use std::fmt;

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            //printing the index too with the respective values there
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}
fn main() {
    let vector = List(vec![1, 4, 54, 5, 1, 4]);
    println!("{}", vector);
}
