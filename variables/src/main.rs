fn main() {
    /*
    let v:i32 = 56;
    v = 98;
    this cod will give error
      - Variables are immutable by default.
     */
    // -- Concept of mutability --
    // let mut v = -98;
    // println!("{}",v);
    // v = 56;
    // println!("{}",v);

    // -- Concept of Shadowing --

    let v = 98;
    println!("{}",v);
    
    let v = "Name";
    println!("{}",v);
}
