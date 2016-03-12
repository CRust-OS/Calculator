use nom::*;
use nom::IResult::*;
use core::convert::From;
use collections::vec;

use core::str;
use core::str::FromStr;
use core::iter::Iterator;

enum Op {
   Plus,
   Minus,
   Mult,
   Div
}

impl<'a> From<&'a [u8]> for Op {
    fn from(s : &'a [u8]) -> Op {
        match s {
            b"+" => Op::Plus,
            b"-" => Op::Minus,
            b"*" => Op::Mult,
            b"/" => Op::Div,
            _ => unreachable!()
        }
    }
}

impl Op {
    fn compute(&self, a : i64, b : i64) -> i64 {
        match self {
            &Op::Plus   => a + b,
            &Op::Minus  => a - b,
            &Op::Mult   => a * b,
            &Op::Div    => a / b
        }
    }
}

named!(parens <i64>, delimited!(
    char!('('),
    expr,
    char!(')')
));

named!(num <i64>, 
    map_res!(
       map_res!(digit, str::from_utf8),
       FromStr::from_str
    )
);

named!(factor <i64>, delimited!(
   opt!(space),
   alt!(num | parens),
   opt!(space)
));

named!(term <i64>, chain!(
    f   : factor ~
    rest: many0!(
            pair!(
                alt!(tag!("*") | tag!("/")),
                factor
            )), 
    ||{
        rest.into_iter().fold(f, |acc, (op, f)|{
            Op::from(op).compute(acc, f)
        })
    }
));

named!(expr<i64>, chain!(
    t   : term ~
    rest: many0!(
            pair!(
                alt!(tag!("+") | tag!("-")),
                term
            )),
    ||{
        rest.into_iter().fold(t, |acc, (op, t)|{
            Op::from(op).compute(acc, t)
        })
    }
));

named!(pub line<i64>, terminated!(expr, eof));
