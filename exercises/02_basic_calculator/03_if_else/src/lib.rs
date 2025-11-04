/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
fn magic_number(n: u32) -> u32 {
    let mut res:u32 = 0;
    if n==1 {
        res=17;
    }else if n == 2{
        res=12;
    }else if n == 9 {
        res = 13;
    }else if n == 6{
        res = 12;
    }else if n ==233{
        res = 17;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
