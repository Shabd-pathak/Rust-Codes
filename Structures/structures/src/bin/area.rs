#[derive(Debug)]
struct Dimension{
  length : u32,
  width : u32,
}
impl Dimension{
  fn area(&self) ->u32
  {
    self.width * self.length
  }
}
fn main()
{
  let user_input = Dimension{
    length : 32,
    width : 16,
  };
  println!("\nDetails entered in dimensions {:?}\n",user_input);
  let rectangle_area = user_input.area();
  println!("Hence the area of rectangle with dimension \n{} and {} is {}",user_input.length,user_input.width,rectangle_area);
}

// fn area(user_dimension:&Dimension) -> u32
// {
//   user_dimension.length * user_dimension.width
// }