//We generally use null pointer in C/C++ but in rust there is no null pointer instead of that we use optional enum
//Optional enum are by default in main


fn main(){
    let x:i8 = 5;

    let y:Option<i8> = Some(5);

    // let sum = x+y;
    //Here will give error because we are adding i8 with option i8

    //instead
    let sum = x + y.unwrap_or(0);
    println!("{}",sum);
    //Here we are done with all the possibilities of option enum

    //Whatif
    // let y:Option<i8> = None;
    //Then sum == 5



}