enum Position{
    SuperUser(Person)
    Artist(Person)
    User(Person)
}
struct Person{
    name:String,
    passwort:String,

}

fn main() {
    println!("Hello, world!");
}
