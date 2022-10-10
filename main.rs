use crate::variable::User;

mod variable;
fn main() {
    println!("hello world");
    let mut u = User{
        id: 1,
        name:String::from("first")
    };
    u.set(10);
    println!("{}", u.get());
    println!(r#"{:?}"#, u)
}
