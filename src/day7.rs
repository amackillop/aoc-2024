use std::fmt::Debug;

use nom::{
    IResult, bytes::complete::tag, character::complete::space1, multi::separated_list1,
    sequence::separated_pair,
};
use rayon::prelude::*;

use crate::{parse_input, parse_number};

#[derive(Debug)]
struct Equation {
    values: Vec<usize>,
    test_value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

pub fn part_1(input: &str) -> usize {
    let equations = input.lines().map(parse_equation).collect::<Vec<_>>();
    solution(&equations, &[Operator::Add, Operator::Multiply])
}

pub fn part_2(input: &str) -> usize {
    let equations: Vec<Equation> = input.lines().map(parse_equation).collect();
    solution(&equations, &[
        Operator::Add,
        Operator::Multiply,
        Operator::Concat,
    ])
}

fn solution(equations: &[Equation], operators: &[Operator]) -> usize {
    equations
        .par_iter()
        .filter_map(|Equation { values, test_value }| {
            for combo in combinations(values.len() - 1, operators) {
                if evaluate(values, &combo) == *test_value {
                    return Some(*test_value);
                }
            }
            None
        })
        .sum()
}

fn evaluate(values: &[usize], operators: &[Operator]) -> usize {
    debug_assert!(
        values.len() >= 2 && operators.len() == values.len() - 1,
        "Invalid input"
    );
    let mut initial = values[0];
    for (&value, &operator) in values[1..].iter().zip(operators.iter()) {
        match operator {
            Operator::Add => initial += value,
            Operator::Multiply => initial *= value,
            Operator::Concat => initial = initial * 10_usize.pow(value.ilog10() + 1) + value,
        }
    }
    initial
}

// Get the possible combinations of operators for a given number of operands
fn combinations(n: usize, operators: &[Operator]) -> Box<dyn Iterator<Item = Vec<Operator>> + '_> {
    if n == 0 {
        Box::new(std::iter::once(Vec::new()))
    } else {
        Box::new(operators.iter().flat_map(move |&op| {
            combinations(n - 1, operators).map(move |mut combo| {
                combo.push(op);
                combo
            })
        }))
    }
}

fn parse_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_number)(input)
}

fn parse_equation(line: &str) -> Equation {
    let (test_value, list) = parse_input(line, separated_pair(parse_number, tag(": "), parse_list));
    Equation {
        test_value,
        values: list,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(7);
        // assert_eq!(part_1(&input), 3749);
        assert_eq!(part_1(&input), 7885693428401);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(7);
        // assert_eq!(part_2(&input), 11387);
        assert_eq!(part_2(&input), 348360680516005);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = read_input(7);
        let equations: Vec<Equation> = input.lines().map(parse_equation).collect();
        let operators = [Operator::Add, Operator::Multiply, Operator::Concat];

        b.iter(|| solution(&equations, &operators));
    }

    #[bench]
    fn bench_combinations(b: &mut Bencher) {
        let operators = vec![Operator::Add, Operator::Multiply, Operator::Concat];
        b.iter(|| combinations(5, &operators).count());
    }

    #[test]
    fn test_combinations_0() {
        let operators = vec![Operator::Add, Operator::Multiply];
        let combos = combinations(0, &operators).collect::<Vec<_>>();
        assert_eq!(combos, vec![vec![]],);
    }

    #[test]
    fn test_combinations_1() {
        let operators = vec![Operator::Add, Operator::Multiply];
        let combos = combinations(1, &operators).collect::<Vec<_>>();
        assert_eq!(combos, vec![vec![Operator::Add], vec![Operator::Multiply]],);
    }

    #[test]
    fn test_combinations_n() {
        let operators = vec![Operator::Add, Operator::Multiply];
        let combos: Vec<Vec<Operator>> = combinations(3, &operators).collect();
        assert_eq!(combos, vec![
            vec![Operator::Add, Operator::Add, Operator::Add],
            vec![Operator::Multiply, Operator::Add, Operator::Add],
            vec![Operator::Add, Operator::Multiply, Operator::Add],
            vec![Operator::Multiply, Operator::Multiply, Operator::Add],
            vec![Operator::Add, Operator::Add, Operator::Multiply],
            vec![Operator::Multiply, Operator::Add, Operator::Multiply],
            vec![Operator::Add, Operator::Multiply, Operator::Multiply],
            vec![Operator::Multiply, Operator::Multiply, Operator::Multiply],
        ])
    }

    // #[test]
    // fn test_build_expression() {
    //     let values = &[1, 2, 3, 4];
    //     let operators = &[Operator::Add, Operator::Concat, Operator::Multiply];
    //     let expression = build_expression(values, operators).collect::<Vec<_>>();
    //     assert_eq!(expression, vec![
    //         Expr::Value(1),
    //         Expr::Add(2),
    //         Expr::Concat(3),
    //         Expr::Multiply(4),
    //     ])
    // }

    #[test]
    fn test_evaluate() {
        let values = vec![1, 2, 3, 4];
        let operators = vec![Operator::Add, Operator::Concat, Operator::Multiply];
        assert_eq!(evaluate(&values, &operators), 33 * 4);
    }
}
