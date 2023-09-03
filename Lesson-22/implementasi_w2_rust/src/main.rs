// Understanding variables and data types in Rust

fn main() {
    // Variable declaration and initialization
    let age = 25; // integer variable
    let name = "John"; // string variable
    let is_student = true; // boolean variable

    // Data type conversion
    let age_as_string = age.to_string();

    // Printing variables
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Student: {}", is_student);
    println!("Age as String: {}", age_as_string);

    // Working with control flow statements in Rust

    // Using if statement
    if age < 18 {
        println!("You are a minor.");
    } else if age >= 18 && age < 60 {
        println!("You are an adult.");
    } else {
        println!("You are a senior citizen.");
    }

    // Using loop to print numbers from 1 to 5
    let mut counter = 1;
    loop {
        println!("Counter: {}", counter);
        counter += 1;
        if counter > 5 {
            break;
        }
    }

    // Using while loop to print even numbers up to 10
    let mut num = 2;
    while num <= 10 {
        println!("Even number: {}", num);
        num += 2;
    }

    // Using for loop to iterate over a range and print squares
    for i in 1..=5 {
        let square = i * i;
        println!("Square of {}: {}", i, square);
    }
}