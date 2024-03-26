#[derive(Debug)]
struct PersonDetails{
    username:String,
    e_mail:String,
    age:u64,
    work_experience:u32,
}



fn main() {
    let user1 = PersonDetails{
        username : String::from("Shabd Pathak"),
        e_mail:String::from("shadygen005@gmail.com"),
        age:18,
        work_experience:1,
    };
    let user2 = PersonDetails{
        username:String::from("Raghav"),
        e_mail:String::from("raghav005@gmail.com"),
        ..user1
       };

    println!("{:?}",user2);
       let user3 = build_user(String::from("user@gmail.com"),String::from("userme"));

}

fn build_user(e_mail:String,username:String) ->PersonDetails{
    PersonDetails 
    //field-init syntax  
    {username,
    e_mail,
    age:12,
    work_experience:1,
    }
}