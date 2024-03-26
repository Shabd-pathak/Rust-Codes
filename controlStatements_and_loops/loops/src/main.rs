use std::io;
fn main() {
    println!("Please enter the number for which you want the table: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to take input");
    let number:u32 = number.trim().parse().expect("Please enter a number");

    let mut number2 = String::new();
    println!("Please enter the multiple: ");
    io::stdin()
        .read_line(&mut number2)
        .expect("Invalid Input");

    let number2:u32 = number2.trim().parse().expect("Please enter a integer");
    let mut count = 1;
    loop{
 
        println!("{} x {} = {}",number,count,number*count);
        count = count+1;
        if count>number2
        {
            break;
        }

    }
}