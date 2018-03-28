use std::collections::HashSet;

struct Rational {
    numerator: i64,
    denominator: i64,
}

fn calculate(left: &Rational, right: &Rational, op: char) -> Rational {
    let ln = left.numerator;
    let ld = left.denominator;
    let rn = right.numerator;
    let rd = right.denominator;

    let n = match op {
        '+' => ln * rd + rn * ld,
        '-' => ln * rd - rn * ld,
        '*' => ln * rn,
        '/' => ln * rd,
        _ => panic!("unknown operator"),
    };
    let d = match op {
        '+' => ld * rd,
        '-' => ld * rd,
        '*' => ld * rd,
        '/' => ld * rn,
        _ => panic!("unknown operator"),
    };

    let r = Rational {
        numerator: n,
        denominator: d,
    };
    return r;
}

fn calculate_expression(expression_string: &str) -> bool {
    let mut stack = vec![];
    for c in expression_string.chars() {
        match c {
            '+' | '-' | '*' | '/' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(calculate(&left, &right, c));
            }
            '0'...'9' => {
                // expected '0' - '9'
                stack.push(Rational {
                    numerator: c as i64 - '0' as i64,
                    denominator: 1,
                });
            }
            _ => panic!("unkwown character"),
        }
    }
    let result = stack.pop().unwrap();
    if result.numerator == result.denominator * 10 {
        return true;
        //println!("{}", expression_string);
    }
    return false;
}

fn solve(numbers: &[i32], operators: &str) {
    // Make possible numbers,
    // like 1234 1243 1324 1342 1423 1432 2134 2143 2314 2341 2413 2431
    //      3124 3142 3214 3241 3412 3421 4123 4132 4213 4231 4312 4321
    let mut number_set: HashSet<String> = HashSet::new();
    {
        let len = numbers.len();
        let mut bin = vec![0; len]; // flag 1 = used / 0 = unused
        let mut number_string = "".to_string();
        fn make_numbers(
            numbers: &[i32],
            bin: &mut Vec<i32>,
            number_string: &mut String,
            number_set: &mut HashSet<String>,
        ) {
            if number_string.len() == numbers.len() {
                if !number_set.contains(number_string) {
                    number_set.insert(number_string.clone());
                }
                return;
            }
            for i in 0..numbers.len() {
                if bin[i] == 1 {
                    continue;
                }
                bin[i] = 1;
                number_string.push(('0' as u8 + numbers[i] as u8) as char);
                make_numbers(numbers, bin, number_string, number_set);
                number_string.pop();
                bin[i] = 0;
            }
        }
        make_numbers(numbers, &mut bin, &mut number_string, &mut number_set);
    }
    // Inject operators,
    // like 1234+++ 1234+-- 123+4++ 12*34*+ ....
    let mut expression_set: HashSet<String> = HashSet::new();
    for mut number_string in number_set {
        let mut result = "".to_string();
        fn inject_operators(
            number_string: &mut String,
            operator: &str,
            result: &mut String,
            number_count: i32,
            operator_count: i32,
            expression_set: &mut HashSet<String>,
        ) {
            if number_string.len() == 0 && number_count == operator_count + 1 {
                if !expression_set.contains(result) {
                    expression_set.insert(result.clone());
                }
                return;
            }
            if number_count > operator_count + 1 {
                for c in operator.chars() {
                    result.push(c);
                    inject_operators(
                        number_string,
                        operator,
                        result,
                        number_count,
                        operator_count + 1,
                        expression_set,
                    );
                    result.pop();
                }
            }
            if number_string.len() > 0 {
                let c = number_string.pop().unwrap();
                result.push(c);
                inject_operators(
                    number_string,
                    operator,
                    result,
                    number_count + 1,
                    operator_count,
                    expression_set,
                );
                result.pop();
                number_string.push(c);
            }
        }
        inject_operators(
            &mut number_string,
            operators,
            &mut result,
            0,
            0,
            &mut expression_set,
        );
    }
    for mut expression_string in expression_set {
        let result = calculate_expression(&expression_string);
        if result {
            println!("{}", expression_string);
        }
    }
}

fn main() {
    let numbers = &[1, 1, 5, 8];
    solve(numbers, "+-*/");
}
