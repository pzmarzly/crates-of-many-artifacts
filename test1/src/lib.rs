pub fn library() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_library {
    use super::*;

    #[test]
    fn test_library() {
        println!("Hello, world!");
    }
}
