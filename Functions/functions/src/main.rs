//Sum o two numbers
fn main() {
    let num1 = 93;
    let num2 = 54;

    let sum = sum(num1, num2);
    println!("{}",sum);
}

// fn sum(a:i32,b:i32) -> i32{
//         a+b
// }

// fn sum(a:i32,b:i32) -> i32{
//     return a+b
// }


fn sum(a:i32,b:i32) -> i32{
    let sum = a+b;
    return sum;
}
