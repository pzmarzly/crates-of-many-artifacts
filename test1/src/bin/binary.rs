fn binary() {
    println!("Hello, world!");
}

fn main() {
    binary();
}

#[cfg(test)]
mod test_binary {
    #[test]
    fn test_binary() {
        println!("Hello, world!");
    }
}
