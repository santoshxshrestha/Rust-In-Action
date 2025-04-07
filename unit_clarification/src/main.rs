use std::marker::PhantomData;
// there are the variend that we hearned about till 
// pub trait Add<RHS = Self> {
//     type Output;
//
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// pub trait Add<Output, RHS = Self> {
//     fn add(self, rhs: RHS) -> Output;
// }

pub trait Add<RHS = Self,Output  = Self > {
    fn add(self, rhs: RHS) -> Output;
}

impl <U>  Add for T<U> {
    type Output = T<U>;
}

enum Inch {}
enum Mn {}

struct Lenght<Unit>(f64, PhantomData<Unit>);

impl <Unit> Add for Lenght<Unit> {
    type Output = Lenght<Unit>;

    fn add(self, rhs: Lenght<Unit>) -> Lenght<Unit> {
        Lenght(self.0+ rhs.0, PhantomData)
    }
    
}
fn main() {

}
