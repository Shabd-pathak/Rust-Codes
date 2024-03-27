// fn main()
// {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// fn plus_one(x :Option<i32>) ->Option<i32>
// {
//     match x{
//         None =>None,
//         Some(x) => Some(x+1),
//     }
// }

// fn plus_one(x:Option<i32>) -> Option<i32>
// {
//     match x{
//         Some(i) =>Some(i+1),
//         _ => None,
//     }
// }

//Concept of if let

fn main(){
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("Three")
    }
    
}
