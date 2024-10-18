use std::ops::Not;

const END_OPERATION_CHAR: [char; 1] = ['='];
fn main() {
    let expression = "3 * 4 - 9 / 3 + 3.0 + 3 / 3 * 12 * 30 + 2.0^2.0";
    let expr = expr_parser(expression);
    let expr = expr_solver(expr);

    if let Token::Number(value) = expr[0] {
        println!("Expression: {expression}\nResult = {}", value);
    }
}

fn expr_solver(mut expr: Vec<Token>) -> Vec<Token> {
    //println!("{:?}", &expr);
    //println!("Solving multiplications and divisions");
    //solve_operations(&mut expr, Operator::Div);
    //solve_operations(&mut expr, Operator::Mul);
    //solve_operations(&mut expr, Operator::Add);
    //solve_operations(&mut expr, Operator::Sub);
    
    solve_operations(&mut expr, Operator::Pow);
    solve_muls_and_divs(&mut expr);
    solve_adds_and_subs(&mut expr);
    //println!("{:?}", &expr);
    expr
}

fn solve_operations(expr: &mut Vec<Token>, operation_type: Operator){
    let operator_indexes = expr
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if let Token::Operator(op) = token {
                if op == &operation_type {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    operator_indexes
        .into_iter()
        .enumerate()
        .for_each(|(n, index)| {
            let index = index - 1 - n * 2;

            let first_number = expr.remove(index);

            let operator = expr.remove(index);

            let second_number = expr.remove(index);

            let operation_result = Operation {
                first_number,
                operator,
                second_number,
            }
            .eval();

            expr.insert(index, Token::Number(operation_result));
        });
}

fn solve_muls_and_divs(expr: &mut Vec<Token>) {
    let divs_and_muls_indexes = expr
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if let Token::Operator(op) = token {
                if op == &Operator::Div || op == &Operator::Mul {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    divs_and_muls_indexes
        .into_iter()
        .enumerate()
        .for_each(|(n, index)| {
            let index = index - 1 - n * 2;

            let first_number = expr.remove(index);

            let operator = expr.remove(index);

            let second_number = expr.remove(index);

            let operation_result = Operation {
                first_number,
                operator,
                second_number,
            }
            .eval();

            expr.insert(index, Token::Number(operation_result));
        });
}
fn solve_adds_and_subs(expr: &mut Vec<Token>) {
    let adds_and_subs_indexes = expr
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if let Token::Operator(op) = token {
                if op == &Operator::Add || op == &Operator::Sub {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    adds_and_subs_indexes
        .into_iter()
        .enumerate()
        .for_each(|(n, index)| {
            let index = index - 1 - n * 2;

            let first_number = expr.remove(index);

            let operator = expr.remove(index);

            let second_number = expr.remove(index);

            let operation_result = Operation {
                first_number,
                operator,
                second_number,
            }
            .eval();

            expr.insert(index, Token::Number(operation_result));
            //println!("{:?}", &expr);
        });
}

fn expr_parser(expr: &str) -> Vec<Token> {
    let operations = expr
        .chars()
        .filter(|x| x.is_ascii_digit().not() && x != &' ' && x != &'.')
        .chain(END_OPERATION_CHAR.into_iter());
    let numbers = expr.split(OPERATIONS).map(parse_number);

    let tokenized_expression = numbers
        .zip(operations)
        .map(|(number, op)| [Token::tokenize_number(number), Token::tokenize_op(op)].into_iter())
        .flatten()
        .collect::<Vec<_>>();

    tokenized_expression
    //println!("{tokenized_expression:?}");
}
fn parse_number(val: &str) -> Number {
    val.trim()
        .parse()
        .expect("Bad input: tried to parse to Number type")
}

#[derive(Debug)]
enum Token {
    Operator(Operator),
    Number(Number),
}

impl Token {
    fn tokenize_number(val: Number) -> Self {
        Self::Number(val)
    }
    fn tokenize_op(op: char) -> Self {
        Self::Operator(Operator::tokenize_op(op))
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Equal,
}

type Number = f64;
const OPERATIONS: [char; 5] = ['+', '-', '/', '*', '^'];
impl Operator {
    fn tokenize_op(op: char) -> Self {
        match op {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '/' => Operator::Div,
            '*' => Operator::Mul,
            '^' => Operator::Pow,
            '=' => Operator::Equal,
            _ => panic!("Operation not recognized: {op}"),
        }
    }
}

#[derive(Debug)]
struct Operation {
    first_number: Token,
    operator: Token,
    second_number: Token,
}
impl Operation {
    fn eval(self) -> Number {
        match (self.first_number, self.operator, self.second_number) {
            (Token::Number(number1), Token::Operator(Operator::Add), Token::Number(number2)) => {
                number1 + number2
            }
            (Token::Number(number1), Token::Operator(Operator::Sub), Token::Number(number2)) => {
                number1 - number2
            }
            (Token::Number(number1), Token::Operator(Operator::Mul), Token::Number(number2)) => {
                number1 * number2
            }
            (Token::Number(number1), Token::Operator(Operator::Div), Token::Number(number2)) => {
                number1 / number2
            }
            (Token::Number(number1), Token::Operator(Operator::Pow), Token::Number(number2)) => {
                number1.powf(number2)
            }
            (_, _, _) => unimplemented!(),
        }
    }
}
