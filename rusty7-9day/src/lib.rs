#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn it_fails() {
        panic!("you are going to fail")
    }

    #[test]
    fn call_simple_add() {
        assert!(simple_add());
    }

    #[test]
    fn not_equal() {
        let result = 2 + 2;
        assert_ne!(result, 5);
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("this one panics")
    }
}

fn simple_add() -> bool {
    if 2 + 2 == 4 {
        true
    } else {
        false
    }
}
