fn example_binary() {
    println!("Hello, world!");
}

fn main() {
    example_binary();
}

#[cfg(test)]
mod test_example_binary {
    #[test]
    fn test_example_binary() {
        println!("Hello, world!");
    }
}
