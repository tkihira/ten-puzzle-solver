use std::collections::HashSet;

fn calculate(left: f64, right: f64, op: char) -> f64 {
    let r = match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!("unknown operator"),
    };
    return r;
}

fn calculate_expression(expression_string: &str, goal: i32) -> (bool, bool) {
    let mut stack = vec![];
    let mut difficulty = false;
    for c in expression_string.chars() {
        match c {
            '+' | '-' | '*' | '/' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let r = calculate(left, right, c);

                if c == '/' {
                    if (r as i64 as f64) != r {
                        difficulty = true;
                    }
                }
                stack.push(r);
            }
            '0'...'9' => {
                stack.push((c as i64 - '0' as i64) as f64);
            }
            _ => panic!("unkwown character"),
        }
    }
    let result = stack.pop().unwrap();
    if goal as f64 - 0.00001 < result && result < goal as f64 + 0.00001 {
        return (true, difficulty);
    }
    return (false, false);
}

fn solve(numbers: &[i32], operators: &str, goal: i32) -> (Vec<String>, String, bool) {
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
    {
        // solve and gather data
        let mut entire_difficulty = true;
        let mut example_solution = "".to_string();
        let mut solutions = vec![];
        for mut expression_string in expression_set {
            let (result, difficulty) = calculate_expression(&expression_string, goal);
            if result {
                entire_difficulty &= difficulty;
                if difficulty {
                    if example_solution.len() == 0 {
                        example_solution = expression_string.clone();
                    }
                } else {
                    example_solution = expression_string.clone();
                }
                solutions.push(expression_string);
            }
        }
        return (solutions, example_solution, entire_difficulty);
    }
}

fn main() {
    for i0 in 0..10 {
        for i1 in i0..10 {
            for i2 in i1..10 {
                for i3 in i2..10 {
                    let (solutions, example_solution, difficulty) =
                        solve(&[i0, i1, i2, i3], "+-*/", 10);
                    if solutions.len() > 0 {
                        let numbers = format!("{}{}{}{}", i0, i1, i2, i3);
                        print!("{:>5}:{}:{}", solutions.len(), numbers, example_solution);
                        if difficulty {
                            print!("  !");
                        }
                        println!();
                    }
                }
            }
        }
    }

    //let numbers = &[6, 6, 6, 9, 9];
    //solve(numbers, "+-*/");
}
