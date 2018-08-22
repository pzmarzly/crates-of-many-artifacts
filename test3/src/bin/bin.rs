include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());
}

#[test]
fn test_binary() {
    println!("{}", message());
}
