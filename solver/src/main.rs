use std::collections::HashSet;

struct Rational {
    numerator: i64,
    denominator: i64
}

const operations: [char;4] = ['+', '-', '*', '/'];

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

fn solve(numbers: &[i32])  {
    let mut numberSet: HashSet<String> = HashSet::new();
    {
        let len = numbers.len();
        let mut bin = vec![0; len];
        let mut numberString = "".to_string();
        fn make_numbers(numbers: &[i32], bin: &mut Vec<i32>, numberString: &mut String, numberSet: &mut HashSet<String>) {
            if numberString.len() == numbers.len() {
                if !numberSet.contains(numberString) {
                    numberSet.insert(numberString.clone());
                }
                return;
            }
            for i in 0..numbers.len() {
                if bin[i] == 1 {
                    continue;
                }
                bin[i] = 1;
                numberString.push(('0' as u8 + numbers[i] as u8) as char);
                make_numbers(numbers, bin, numberString, numberSet);
                numberString.pop();
                bin[i] = 0;
            }
        }
        make_numbers(numbers, &mut bin, &mut numberString, &mut numberSet);
    }
    for numberString in numberSet {
        println!("{}", numberString);
    }
}

fn main() {
    let numbers = &[1, 1, 5, 8];
    solve(numbers);
}
