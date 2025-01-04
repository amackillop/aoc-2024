use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};
use regex::Regex;

use crate::{parse_input, parse_number};

type Program = Vec<Expr>;

#[derive(Debug)]
enum Expr {
    Mul(usize, usize),
    Do,
    Dont,
}

pub fn part_1(input: &str) -> usize {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    solution(input, &regex)
}

pub fn part_2(input: &str) -> usize {
    let regex = Regex::new(r"don't\(\)|do\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    solution(input, &regex)
}

/// Parse the program from the corrupted input and interpret it.
/// The regex is used to filter the corrupted input
fn solution(input: &str, regex: &Regex) -> usize {
    let input = scan_input(input, regex);
    let program = parse_program(&input);
    interpret(&program)
}

/// Find all the mul(a,b) in the input where a and b are numbers at most 3 digits long
fn scan_input(input: &str, regex: &Regex) -> String {
    regex.find_iter(input).map(|m| m.as_str()).collect()
}

/// Parse mul(a,b) into a tuple of the two numbers to multiply
fn mul_expr(input: &str) -> IResult<&str, Expr> {
    let (rem, (a, b)) = delimited(
        tag("mul("),
        separated_pair(parse_number, tag(","), parse_number),
        tag(")"),
    )(input)?;
    Ok((rem, Expr::Mul(a, b)))
}

/// Parse do() into a Do expression
fn do_expr(input: &str) -> IResult<&str, Expr> {
    let (rem, _) = tag("do()")(input)?;
    Ok((rem, Expr::Do))
}

/// Parse don't() into a Dont expression
fn dont_expr(input: &str) -> IResult<&str, Expr> {
    let (rem, _) = tag("don't()")(input)?;
    Ok((rem, Expr::Dont))
}

/// Parse an expression from the input
fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((do_expr, dont_expr, mul_expr))(input)
}

/// Parse the program from the input
fn parse_program(input: &str) -> Program {
    parse_input(input, many1(parse_expr))
}

/// Interpret the program.
/// Do and Dont expressions toggle whether to execute the Mul expressions that follow
fn interpret(program: &Program) -> usize {
    program
        .iter()
        .fold((true, 0), |acc, expr| match expr {
            Expr::Mul(a, b) => (acc.0, if acc.0 { acc.1 + a * b } else { acc.1 }),
            Expr::Do => (true, acc.1),
            Expr::Dont => (false, acc.1),
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use crate::test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(3);
        assert_eq!(part_1(&input), 161085926);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(3);
        assert_eq!(part_2(&input), 82045421);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = read_input(3);
        let regex = Regex::new(r"don't\(\)|do\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
        b.iter(|| solution(&input, &regex));
    }
}
