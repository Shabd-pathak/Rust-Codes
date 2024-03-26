use std::io;
fn main() {
    let mut number1 =String::new();
    println!("Enter the number you want to compare: ");

    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to take input");

        let number1: u32 = number1.trim().parse().expect("Please type a number: ");

    let mut operation =String::new();
    println!("Enter the operation(+, -, *, /): ");
    
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to take input");
    

    let mut number2 =String::new();
    println!("Enter the number for comparing: ");
    
    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to take input");
    
        let number2: u32 = number2.trim().parse().expect("Please type a number: ");

    if operation.trim() == "+" {
        println!("Sum of {} and {} is {}",number1,number2,number1 + number2);
    }
    else if operation.trim() == "-"{
        println!("Sum of {} and {} is {}",number1,number2,number1 - number2);
    }
    else if operation.trim() == "*"{
        println!("Sum of {} and {} is {}",number1,number2,number1 * number2);
    }
    else if operation.trim() == "/"{
        println!("Sum of {} and {} is {}",number1,number2,number1 / number2);
    }
    else{
        println!("This code will only work for given operation.\nPlease try again");
    }
        
}
