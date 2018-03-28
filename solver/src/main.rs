struct Rational {
    numerator: i64,
    denominator: i64
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn calculate(left: &Rational, right: &Rational, op: i32) -> Rational {
    let ln = left.numerator;
    let ld = left.denominator;
    let rn = right.numerator;
    let rd = right.denominator;
    
    let n = match op {
        0 => ln * rd + rn * ld,
        1 => ln * rd - rn * ld,
        2 => rn * ld - ln * rd,
        3 => ln * rn,
        4 => ln * rd,
        5 => rn * ld,
        _ => 999999999999999999
    };
    let d = match op {
        0 => ld * rd,
        1 => ld * rd,
        2 => ld * rd,
        3 => ld * rd,
        4 => ld * rn,
        5 => rd * ln,
        _ => 9999999999999999999
    };

    let r = Rational { numerator: n, denominator: d};
    return r;
}

fn reducer(rationals: &[Rational], solution:&mut Vec<char>) -> bool {
    println!("len:{}", rationals.len());
    if rationals.len() == 1 {
        let rational = &rationals[0];
        println!("{}/{}", rational.numerator, rational.denominator);
        if rational.denominator != 0 && rational.numerator == rational.denominator * 10 {
            return true;
        }
        return false;
    }
    for i in 0..rationals.len() - 1 {
        for j in i + 1..rationals.len() {
            // push two other numbers first
            let mut new_rationals = vec![];
            for k in 0..rationals.len() {
                if i == k || j == k {
                    continue;
                }
                new_rationals.push(Rational{..rationals[k]});
            }
            for l in 0..6 {
                let r = calculate(&rationals[i], &rationals[j], l);
                new_rationals.push(r);
                if reducer(&new_rationals, solution) {
                    return true;
                }
                new_rationals.pop();
            }
        }
    }
    return false;
}

fn solve(numbers: &[i64])  {
    let mut rationals = vec![];
    for i in 0..numbers.len() {
        let rational = Rational { numerator: numbers[i], denominator: 1 };
        rationals.push(rational);
    }
    let mut solution = vec![];
    let result = reducer(&rationals, &mut solution);
    println!("{}", result);
}

fn main() {
    let numbers = &[9, 9, 9, 9];
    solve(numbers);
}
