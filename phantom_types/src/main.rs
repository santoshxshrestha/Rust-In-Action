use std::marker::PhantomData;
struct USD;
struct EUR;

#[derive(Debug)]
struct Money<Currency> {
    amount: f64,
    _phantom: PhantomData<Currency>,
}

fn dollars(amount: f64) -> Money<USD> {
    Money { amount, _phantom: PhantomData }

}

fn euros(amount: f64) -> Money<EUR> {
    Money { amount, _phantom: PhantomData}
}

#[allow(dead_code)]
impl<C> Money<C> {
    fn add(self, other: Money<C>) -> Money<C> {
        Money { amount: self.amount + other.amount,
        _phantom: PhantomData
        }
    }
}

 fn main() {
     let payment = dollars(29.9);
     let price = euros(52.5);

     //here I am directly accessing the amount parameter so I am not getting 
     //the errors
     let total = payment.amount + price.amount;
     println!("{total}");
     
     {
         //here after the impl block the phantom data is different of the 
         //both variable so we can't do the addition thing out there 
         // let total = payment.add(price);
     }
}
    
