pub fn fn3() -> u8 {
    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_three() {
        assert_eq!(fn3(), 3);
        ()
    }
}
