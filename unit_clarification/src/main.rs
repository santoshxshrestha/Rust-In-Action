// there are the variend that we hearned about till 
// pub trait Add<RHS = Self> {
//     type Output;
//
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// pub trait Add<Output, RHS = Self> {
//     fn add(self, rhs: RHS) -> Output;
// }

use std::ops::Add;
use std::marker::PhantomData;

// Unit marker types
#[derive(Debug)]
enum Inch {}

enum Mm {} // millimeter

// Generic Length type with phantom data for units
#[derive(Debug)]
struct Length<Unit>(f64, PhantomData<Unit>);

// Implement Add for same units
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let length1 = Length::<Inch>(5.0, PhantomData);
    let length2 = Length::<Inch>(3.0, PhantomData);
    let total = length1 + length2;
    println!("Total length in inches: {:?}", total);

    // Uncommenting the lines below would give a compile-time error
    // let length_mm = Length::<Mm>(10.0, PhantomData);
    // let invalid = length1 + length_mm;
}

