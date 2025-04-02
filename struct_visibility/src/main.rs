#![allow(unused)]
mod bank {
    pub struct BankAccount {
        account_number: String,
        owner_name: String,
        balance: f64,
    }

    impl BankAccount{
        pub fn new(account_number: String, owner_name: String, initial_deposit: f64) -> Result<BankAccount, String>{
            if initial_deposit < 100.0 {
                return Err("Initial deposit must be at least $100".to_string());
            }
            Ok(BankAccount{
                account_number,
                owner_name,
                balance: initial_deposit,
            })
        }

        pub fn get_balance(&self) -> f64 {
            self.balance
        }

        pub fn deposit(&mut self, amount: f64) -> Result<f64, String> {
            if amount <= 0.0 {
                return Err("Deposit amount must be positive".to_string());
            }

            self.balance+=amount;
            Ok(self.balance)

        }

        pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {

            if amount <=0.0 {
                return Err("Withdraw amount must be positive".to_string());
            }

            if amount > self.balance{
                return Err("Insufficient funds".to_string());
            }

            self.balance -= amount;
            Ok(self.balance)
        }

        pub fn transfer(&mut self, to_account: &mut BankAccount, amount: f64)-> Result<(), String>{
            match self.withdraw(amount){
                Ok(_)=> {
                    match to_account.deposit(amount) {
                        Ok(_) => Ok(()),
                        Err(e)=> {
                            self.balance += amount;
                            Err(e)
                        }

                    }
                },
                Err(e) => Err(e)
            }


        }
    }

}
fn main() {
    let mut alice_account = match bank::BankAccount::new(
        "A12345".to_string(),
        "Alice Smith".to_string(),
        1000.0
    ){
        Ok(account) => account,
        Err(e) => {
            println!("Error creating account: {}",e);
            return;
        }
    };


    //you could do the following thing like 
    //if there wasn't any module out there for the BankAccount
    //since we created a new module out there named bank so it is in now total incuptulation
    // println!("Stealing the data of the alice {}",alice_account.balance());
    //Why does this work?
    // Rust enforces privacy at the module level.
    // Since our struct BankAccount and our main function exist in the same module (the crate root, in this case),
    // the main function has full access to the private fields of BankAccount.

    let mut bob_account = match bank::BankAccount::new(
        "B21413".to_string(),
        "Bob Johnson".to_string(),
        1200.0
    ){
        Ok(account) => account,
        Err(e)=> {
            println!("Error creating account: {}",e);
            return;
        }
    };

    //here we can't directly create the instance because the structre is private

    println!("Alice's balance: ${:.2}", alice_account.get_balance());
    println!("Bob's balance:${:.2}",bob_account.get_balance());

    match alice_account.deposit(200.0) {
        Ok(new_balance) => println!("Deposited successful. Alice's new balance: ${:.2}",new_balance),
        Err(e) => println!("Deposit failed: {}",e)
    }

    match alice_account.transfer(&mut bob_account, 300.0){
        Ok(_) => {
            println!("Transfer successful.");
            println!("Alice's balance: ${:.2}", alice_account.get_balance());
            println!("Bob's balance: ${:.2}", bob_account.get_balance());
        },
        Err(e) =>println!("Transfer failed: {}",e)
    }

    match alice_account.withdraw(1000.0){
        Ok(new_balance) => println!("withdrawl successful. New balance: ${:.2}",new_balance),
        Err(e) => println!("withdrawl failed: {}", e)

    }

}
