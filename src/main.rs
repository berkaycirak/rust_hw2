use std::fmt::Debug;

fn main() {
    let mut main_bank_account = BankAccount {
        account_number:1,
        holder_name:"Berkay".to_string(),
        balance:50
    };
    
    let mut other_bank_account = BankAccount {
        account_number:2,
        holder_name:"Joe".to_string(),
        balance:50
    };

    main_bank_account.deposit(10);
    other_bank_account.withdraw(15);

 


    println!("Main Bank Balance {}",main_bank_account.get_balance());
    println!("Other Bank Balance {:?}",other_bank_account.get_balance());
    println!("Main Bank Account {:?}",main_bank_account);
    println!("Other Bank Account {:?}",other_bank_account);

    
}

trait Account {
    fn deposit(&mut self,amount:u32);
    fn withdraw(&mut self,amount:u32);
    fn get_balance(&mut self)->u32;
    
}
#[derive(Debug)]
struct BankAccount {
    account_number:u32,
    holder_name:String,
    balance:u32

}

impl Account for BankAccount {
    fn deposit(&mut self,amount:u32) {
        self.balance +=amount;
    }

    fn withdraw(&mut self,amount:u32) {
        self.balance -=amount
    }

    fn get_balance(&mut self)->u32 {
        self.balance
        
    }
}