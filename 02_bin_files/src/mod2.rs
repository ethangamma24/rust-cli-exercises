use crate::mod1::fn1;

pub fn fn2() -> u8 {
    fn1() * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_fn1() {
        let fn1_res = 1;
        assert_eq!((fn1_res * 2), fn2());
        ()
    }
}
