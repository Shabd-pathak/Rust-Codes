enum University{
    Private(String),
    Public(String),
}

impl University{
    fn print_output()
    {
        //code here
    }
}
//WHAT IF WE ARE USING STRUCT for enum

// enum University{
//     Private,
//     Public,
// }
// struct UV{
//     university_type: University,
//     university_name : String,
// }
fn main()
{
    // let user_college = UV{
    //     university_type : Private,
    //     university_name : String::from("XYZ-College"),
    // };

    let user_1_college = University::Private(String::from("XYZ University"));
}

