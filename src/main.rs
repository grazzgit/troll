struct User {
    name: String,
    email: String,
}

impl User {
    fn new(name: &str) -> User {
        return User {
            name: name.to_string(),
            email: format!("{}@example.com", name),
        };
    }
}

fn main() {
    let user = User::new("John");
    println!("user:{}, email:{}", user.name, user.email)
}
