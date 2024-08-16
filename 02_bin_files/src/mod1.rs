pub fn fn1() -> u8 {
    1
}

#[cfg(test)]
mod tests {
    use crate::mod1::fn1;

    #[test]
    fn return_one() {
        assert_eq!(1, fn1());
        ()
    }
}
