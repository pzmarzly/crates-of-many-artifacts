extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn proc_macro(_input: TokenStream) -> TokenStream {
    "println!(\"Hello, world!\")".parse().unwrap()
}

#[cfg(test)]
mod test_proc_macro {
    #[test]
    fn test_proc_macro() {
        println!("Hello, world!");
    }
}
