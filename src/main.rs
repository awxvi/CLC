#[macro_use]
extern crate text_io;

use std::fs::File;
use std::io::prelude::*;

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

fn power_of(){
    /* DEBUG PURPOUSES */
    // println!("You picked: power_of");
    /* END OF BUFFER */

    println!("Please say a whole number and to the power of what: ");
    let first_num_pwrof: i64 = read!();
    let second_num_pwrof: u32 = read!();
    println!("Your result is:  {}", first_num_pwrof.pow(second_num_pwrof));
}

fn sqrt(){
    /* DEBUG PURPOUSES */
    // println!("You picked: sqrt");
    /* END OF BUFFER */

    println!("Please say what number you want to square root: ");
    let first_num_sqrt: f64 = read!();

    println!("Your result is:  {}", first_num_sqrt.sqrt());
}

fn main() {
    println!("Hello and welcome to CLC (command line calculator).");
    println!("What operation would you like to do?");

    let operation: String = read!();

//     if operation == "divide" || operation == "Divide" {
//         divide();
//     } else if operation == "multiplication" || operation == "multiply" || operation == "Multiplication" || operation == "Multiply" {
//         multiplication();
//     } else if operation == "addition" || operation == "add" || operation == "Addition" || operation == "Add" {
//         addition();
//     } else if operation == "subtraction" || operation == "subtract" || operation == "Subtraction" || operation == "Subtract" {
//         subtraction();
//     } else if operation == "power_of" || operation == "Power_of" || operation == "Power_Of"{
//         power_of();
//     } else if operation == "sqrt" || operation == "Sqrt"  || operation == "square_root" || operation == "Square_Root"{
//         sqrt();
//     } else {
//         println!("OOF, failed somewhere.");
//     }
    match operation.as_str(){
        "divide" => divide(),
        "Divide" => divide(),
        "/" => divide(),
        "multiplication" => multiplication(),
        "Multiplication" => multiplication(),
        "*" => multiplication(),
        "addition" => addition(),
        "Addition" => addition(),
        "+" => addition(),
        "subtraction" => subtraction(),
        "Subtraction" => subtraction(),
        "-" => subtraction(),
        "power_of" => power_of(),
        "Power_Of" => power_of(),
        "Power_of" => power_of(),
        "power_Of" => power_of(),
        "^" => power_of(),
        "square_root" => sqrt(),
        "Square_Root" => sqrt(),
        "Square_root" => sqrt(),
        "square_Root" => sqrt(),
        "!" => sqrt(),
        _ => println!("Error, Failed somewhere!")
    }
}