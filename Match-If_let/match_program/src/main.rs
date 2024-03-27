#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    California,
}
enum Coin{
    Penny,
    Nickel,
    Dime(UsState),
}
fn main(){
    value_in_cents(Coin::Dime(UsState::Alaska));
}
fn value_in_cents(coin:Coin) -> u8{
     match coin{
        Coin::Penny => 5,
        Coin::Nickel =>10,
        Coin::Dime(state) =>{
            println!("State Dime from {:?}",state);
            25
        }
     }
}