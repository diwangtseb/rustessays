use crate::variable::{User, Event};

mod variable;
fn main() {
    println!("hello world");
    let mut u = User{
        id: 1,
        name:String::from("first")
    };
    u.set(10,String::from ("张三"));
    println!("{}", u.get());
    println!(r#"{:?}"#, u);
    let run = u.run();
    if let Ok(v) = run{
        println!("{}",v)
    }
    let read = u.read();
    println!("{}",read);
}
