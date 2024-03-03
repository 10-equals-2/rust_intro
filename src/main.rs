use std::io;

fn read_input()
{
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {}", input);
    }
    println!("Goodbye!")
}

fn read_number() -> i32 {
    let mut input = String::new();
    
    println!("Please tell me how many elements you'd like to sum.");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a:i32 =  input.trim().parse().unwrap();
    let mut counter = 0;
    let mut ins:Vec<i32> = vec![];

    while counter <  a {
        counter += 1;
        input.clear();
        println!("Please enter a number");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let b: i32 = input.trim().parse().unwrap();
        ins.push(b);
    }
    summation(ins)/a
}

fn process_numbers(numbers: &[i32]) {
    // initialize sum -> zero
    let mut sum = 0;

    // Iterate over the numbers, adding each one to the sum
    for number in numbers {
        sum += number;
    }

    // Print the sum
    println!("The sum of the numbers is: {}", sum);

    // If the sum is even, print a message
    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}
fn summation(numbers: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for number in numbers {
        result += number;}
    result
}

fn main() {
    println!("The Average of the input numbers is: {}", read_number());
}