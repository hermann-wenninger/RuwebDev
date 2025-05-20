enum Role{
    SuperUser,
    Artist,
    User,
}
#[derive(Debug)]
enum Position {
    SuperUser(Person),
    Artist(Person),
    User(Person),
}
#[derive(Debug)]
struct Person {
    name: String,
    password: String,
}
impl Position{
    pub fn new(role:Role, name:&str,password:&str)->Self {
        let person = Person{
            name:name.to_string(),
            password:password.to_string(),
        };
        match role{
            Role::SuperUser => Position::SuperUser(person),
            Role::Artist => Position::Artist(person),
            Role::User => Position::User(person),
        }
    }
    }

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

   let artist = Position::new(Role::Artist, "Max", "abc123");
   println!("{:?}",artist);
    let superuser = Position::new(Role::SuperUser, "Maximilian", "abcxxx123");
   println!("{:?}",superuser);
   let user = Position::new(Role::User, "Susanna", "abcxxx123fjfjj");
   println!("{:?}",user);
}
