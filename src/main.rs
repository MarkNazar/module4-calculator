use std::io;
fn main() {
    println!("Calculator!");

    let symbol = loop {
        let mut operation = String::new();

        println!("Please input the operation to be use:");
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

          match operation.trim(){
            "+"  => break "+",  
            "-"  => break "-", 
            "*"  => break "*", 
            "/"  => break "/", 
            _ => println!("Please input valid operation: +, -, *, /"),
        }
        };

    let first_number = loop {

        let mut first_number = String::new();
        println!("First Number");
        io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    let first_number: f32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
     };

     break first_number;

    };

    let second_number = loop {

    let mut second_number = String::new();
            println!("Second Number");

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    let second_number: f32 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
     };
     break second_number;
    };

    
    let result = if symbol == "+" {
        first_number + second_number
    } else if symbol == "-" {
         first_number - second_number
     } else if symbol == "*" {
        first_number * second_number
     } else {
        first_number / second_number
     };

    println!("===== Result ====="); 
    println!("{} {} {} = {} ", first_number, symbol, second_number, result);   
    println!("===== Result ====="); 
}       

