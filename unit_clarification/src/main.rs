// there are the variend that we hearned about till 
// pub trait Add<RHS = Self> {
//     type Output;
//
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// pub trait Add<Output, RHS = Self> {
//     fn add(self, rhs: RHS) -> Output;
// }



// pub trait Add<RHS = Self, Output = Self> {
//     fn add(self, rhs: RHS ) -> Output;
// }


#![allow(unused)]
use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug)]
struct Meters;
struct Kilometers;

#[derive(Debug)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Add for Quantity<Unit> {
    type Output = Quantity<Unit>;

    fn add(self, rhs: Quantity<Unit>) -> Quantity<Unit> {
        Quantity { value: self.value + rhs.value,
            _unit: PhantomData,
             }
    }
}

fn main() {
    let dist1 = Quantity::<Meters> { value: 100.0, _unit: PhantomData };
    let dist2 = Quantity::<Meters> { value: 50.0, _unit: PhantomData };

    let total = dist1 + dist2;
    println!("Total distance in meters: {:?}", total);

    let km1 = Quantity::<Kilometers> { value: 2.0, _unit: PhantomData };
    
    // This line will NOT compile (unit mismatch)
    // let error = dist1 + km1;
}
