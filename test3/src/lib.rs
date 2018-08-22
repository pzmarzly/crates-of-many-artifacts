include!(concat!(env!("OUT_DIR"), "/hello.rs"));

#[test]
fn test_library() {
    println!("{}", message());
}
