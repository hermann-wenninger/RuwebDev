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








struct AdminUser {
    username: String,
    password: String
}
struct User {
    username: String,
    password: String
}


trait CanEdit {
    fn edit(&self) {
        println!("admin is editing");
    }
}
trait CanCreate {
    fn create(&self) {
        println!("admin is creating");
    }
}
trait CanDelete {
    fn delete(&self) {
        println!("admin is deleting");
    }
}


trait GetUsername {
    fn get_username(&self) -> &str;
}
trait SetUsername {
    fn set_username(&mut self, username: String);
}

impl GetUsername for User {
    fn get_username(&self) -> &str {
        &self.username
    }
}

impl SetUsername for User {
    fn set_username(&mut self, username: String) {
        self.username = username;
    }
}

impl CanDelete for AdminUser {}
impl CanCreate for AdminUser {}
impl CanEdit for AdminUser {}

impl CanEdit for User {
    fn edit(&self) {
        println!("A standard user {} is editing", 
                 self.username);
    }
}


fn create<T: CanCreate>(user: &T) -> () {
    user.create();
}
fn edit<T: CanEdit>(user: &T) -> () {
    user.edit();
}
fn delete<T: CanDelete>(user: &T) -> () {
    user.delete();
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





    let mut user = User {
        username: String::from("john_doe"),
        password: String::from("password123")
    };

    // Use the getter
    println!("Username: {}", user.get_username());

    // Use the setter
    user.set_username(String::from("jane_doe"));
    println!("Updated Username: {}", user.get_username());

    let admin = AdminUser{
        username: "admin".to_string(), 
        password: "password".to_string()
    };
    let user = User{
        username: "user".to_string(), 
        password: "password".to_string()
    };
    create(&admin);
    edit(&admin);
    edit(&user);
    delete(&admin);
}
