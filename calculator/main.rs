use std::io::{ stdin };

/* TODO:
    1. Implement graphing and checking points.
    2. sqrt().
    3. sqr().
    4. e.
    5. log & ln.
 */

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

fn perform_arithmetic(x: f64, op: char, y: f64) -> f64 {
    println!("Performing: {} {} {}", x, op, y);
    match op {
        '+' => x+y,
        '-' => x-y,
        '*' => x*y,
        '/' => x/y,
        _ => panic!("ERR: terminal symbol: `{}` is not a primitive arithmetic operator.\nAllowed: [`+`, `-`, `*`, `/`]", op),
    }
}

fn is_sym(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/'
}

// Give this function a vec of nums and symbols and it will spit out the resulting math.
fn evaluate(nums: &mut Vec<f64>, syms: &mut Vec<char>) -> f64 {

    let mut total = 0.0;

    // Deal with priorities, *|/, left->right.
    let mut should_loop = true;
    while should_loop {
        should_loop = false;
        for i in 0..syms.len() {
            if syms[i] == '*' || syms[i] == '/' {
                nums[i] = perform_arithmetic(nums[i], syms[i], nums[i+1]);
                // println!("Done: {}", nums[i]);
                nums.remove(i+1);
                syms.remove(i);
                should_loop = true;
                break;
            }
        }
    }

    // Set total to the first element as a starting point.
    if nums.len() > 0 {
        total = nums[0];
    }

    println!("Built: {:?} {:?}", nums, syms);
    for (i, s) in syms.iter().enumerate() {
        total = perform_arithmetic(total, *s, nums[i+1]);
        println!("Done: {}", total);
    }

    total
}

fn check_paren_count(passed_eq: &str) -> bool {
    let mut count: i32 = 0;
    for c in passed_eq.chars() {
        if c == '(' {
            count += 1;
        }
        if c == ')' {
            count -= 1;
        }
        if count < 0 {
            return false;
        }
    }
    count == 0
}

// Build vecs of nums and symbols.
fn parse_equation(passed_eq: &str) -> f64 {

    if passed_eq.trim() == "quit" {
        println!("quiting...");
        return 0.0;
    }

    if !check_paren_count(passed_eq) {
        println!("ERR: unbalanced parenthesis.");
        return 0.0;
    }

    let mut cur_num = String::new();
    let mut parsed_nums = Vec::<f64>::new();
    let mut parsed_syms = Vec::<char>::new();
    let mut paren_count: usize = 0;

    println!("Parsing: {}", passed_eq);
    for c in passed_eq.chars() {
        if c == ')' {
            paren_count -= 1;
            match paren_count {
                0 => {
                    match cur_num.len() {
                        0 => cur_num = "0".to_string(),
                        _ => cur_num = parse_equation(&cur_num.to_string()).to_string(),
                    }
                }
                _ => cur_num.push(c),
            }
        }

        else if c == '(' {
            if paren_count > 0 {
               cur_num.push(c);
            }
            paren_count += 1;
        }

        else if paren_count > 0 {
            cur_num.push(c);
        }

        else if is_sym(c) {
            if cur_num.len() == 0 {
                // Deals with cases where it is just a negative number. e.g. -1.
                cur_num.push('0');
            }
            parsed_syms.push(c);
            parsed_nums.push(cur_num.parse::<f64>().unwrap());
            cur_num.clear();
        }

        else if c.is_ascii_digit() || c == '.' {
            cur_num.push(c);
        }
    }

    // Deal with the last number if it exists.
    if cur_num.len() > 0 {
        parsed_nums.push(cur_num.parse::<f64>().unwrap());
    }

    // Return the evaluation of what was parsed.
    evaluate(&mut parsed_nums, &mut parsed_syms)
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        unimplemented!()
    }
}

fn main() {
    let mut buffer = String::new();
    let mut history = Vec::<f64>::new();
    println!("Type: `quit` to quit the program.");
    while buffer.trim() != "quit" {
        if buffer.trim() == "graph" {
            // TODO: implement graphing.
            unimplemented!()
        }
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Failed to read into buffer.");
        println!("|\nV\n--------------------------------");
        history.push(parse_equation(&buffer));
        if history.len() > 0 {
            print!("\nHistory: [ ");
            for i in &history {
                print!("{} ", i);
            }
            print!("]\n");
        }
        println!("\nResult -> {}\n", history[history.len()-1]);
        println!("--------------------------------\n");
    }
}

