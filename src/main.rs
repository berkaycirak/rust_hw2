use std::{fmt::{Debug, Error}, io::{stdin, stdout, Write}};

// HOMEWORK-1 and HOMEWORK-2 are in that file


fn main() {
    start_operation();
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


enum Operation{
    Add(f64,f64),
    Substract(f64,f64),
    Divide(f64,f64),
    Multiply(f64,f64)
}


fn start_operation(){

    let mut op = String::new();
    let mut numbers = String::new();
   
    print!("Please enter your operation |Add|Substract|Divide|Multiply|: ");
    let _=stdout().flush();
    stdin().read_line(&mut op).expect("Failed to read line");
    print!("Please enter two number with comma for your operation ");
    let _=stdout().flush();
    stdin().read_line(&mut numbers).expect("Failed to read line");

    let number_list :Vec<f64> = numbers.trim().split(",").filter_map(|s| s.trim().parse().ok()).collect();

    let op = op.trim().to_lowercase();
    

    let operation = match op.as_str() {
        "add"=>Operation::Add(number_list[0], number_list[1]),
        "divide"=>Operation::Divide(number_list[0], number_list[1]),
        "multiply"=>Operation::Multiply(number_list[0], number_list[1]),
        "substract"=>Operation::Substract(number_list[0], number_list[1]),
        _ => panic!("Invalid operation. Please enter Add, Subtract, Multiply, or Divide.")
    };


    let result = calculate(operation);

    println!("Your result {}",result)
    

    



}

fn calculate(process:Operation)->f64 {
    match process {
        Operation::Add(a,b) => a + b,
        Operation::Substract(a, b) => a-b,
        Operation::Divide(a, b) => a / b,
        Operation::Multiply(a,b ) => a * b,
    }

}

