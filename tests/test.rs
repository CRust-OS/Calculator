#![allow(unused_imports)]

extern crate calculator;
extern crate nom;

#[cfg(test)]
mod test {
    use nom::IResult;
    use nom::IResult::*;
    use calculator::parser::line;

    fn check(input: &[u8], expected : i64){
        assert_eq!(line(input),IResult::Done(&b""[..], expected));
    }
    #[test]
    fn it_works() {
        check(b"1+2", 3);
    }

    #[test]
    fn parse_num() {
        check(b"1", 1);
    }

    #[test]
    fn parse_parens() {
        check(b"(1123)", 1123);
    }

    #[test]
    fn parse_space() {
        check(b"11 + 2", 13);
    }
}
