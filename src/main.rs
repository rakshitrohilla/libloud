struct User {
    name:String,
    age: u32,
    email:String,

}
fn main () {
   let mut name = String::from("Rakshit");
   let age = 30;
   let email = String::from("Rakshitrohilla@gmail.com");
   let user1 = User {
    name,
    age,
    email,
   
    };
    println!("User : {},Age: {}, Email:{}",user1.name,user1.age,user1.email);
}