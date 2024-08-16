//!   The simplest of simple for Rust programs!
fn main() {
    println!("Hello, world!");
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_nothing() {
        let result = main();
        assert!(result == ());
    }
}
