#[macro_use]
extern crate text_io;

fn divide() {
    /* DEBUG PURPOUSES */
    // println!("You picked: divide");
    /* END OF BUFFER */

    println!("Please say two numbers: ");
    let first_num_divide: f64 = read!();
    let second_num_divide: f64 = read!();
    println!("Your result is:  {}", first_num_divide / second_num_divide);
}

fn multiplication() {
    /* DEBUG PURPOUSES */
    // println!("You picked: multiplication");
    /* END OF BUFFER */

    println!("Please say two numbers: ");
    let first_num_multiplication: f64 = read!();
    let second_num_multiplication: f64 = read!();
    println!("Your result is:  {}", first_num_multiplication * second_num_multiplication);
}

fn addition() {
    /* DEBUG PURPOUSES */
    // println!("You picked: addition");
    /* END OF BUFFER */

    println!("Please say two numbers: ");
    let first_num_addition: f64 = read!();
    let second_num_addition: f64 = read!();
    println!("Your result is:  {}", first_num_addition + second_num_addition);
}

fn subtraction() {
    /* DEBUG PURPOUSES */
    // println!("You picked: subtraction");
    /* END OF BUFFER */

    println!("Please say two numbers: ");
    let first_num_sub: f64 = read!();
    let second_num_sub: f64 = read!();
    println!("Your result is:  {}", first_num_sub - second_num_sub);
}

fn main() {
    println!("Hello and welcome to CLC (command line calculator).");
    println!("What operation would you like to do?");

    let operation: String = read!();

    if operation == "divide" || operation == "Divide" {
        divide();
    } else if operation == "multiplication" || operation == "multiply" || operation == "Multiplication" || operation == "Multiply" {
        multiplication();
    } else if operation == "addition" || operation == "add" || operation == "Addition" || operation == "Add" {
        addition();
    } else if operation == "subtraction" || operation == "subtract" || operation == "Subtraction" || operation == "Subtract" {
        subtraction();
    } else {
        println!("OOF, failed somewhere.");
    }
}