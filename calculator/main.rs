use std::io::{ stdin };
use gnuplot::{ Figure, AxesCommon, Caption, Graph };

/* TODO:
    1. Implement graphing and checking points.                       DONE.
    2. sqrt().
    3. ^x.                                                           DONE.
    4. e.
        e = lim (1+(1/n))^n
           n->inf
    5. log & ln.
    6. Support for not supplying an operator and defaulting to *.
    7. Change parsing for graphs. Always wrap the number in ().
    8. Make ^ more efficient.
    9. Use external library/API to draw a visual of a graph.         DONE.
 */

struct Point {
    x: f64,
    y: f64,
}

struct GraphInput {
    y: String,
    amt: i32,
}

impl Point {
    // Point constructor.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, }
    }
}

impl GraphInput {
    // GraphInput constructor.
    pub fn new(y: String, amt: i32) -> Self {
        Self { y, amt, }
    }
}

// Depending on the operator, it will perform appropriate arithmetic.
fn perform_arithmetic(x: f64, op: char, y: f64, exp: Option<i32>, verbose: bool) -> f64 {
    if verbose {
        println!("Performing: {} {} {}", x, op, y);
    }

    match op {
        '+' => x+y,
        '-' => x-y,
        '*' => x*y,
        '/' => x/y,
        '%' => x%y,
        '^' => {
            // Calculate exponent.
            if exp.is_none() { panic!("Not a valid exponent.") }
            let mut total: f64 = x;
            for _ in 0..exp.unwrap_or(-0)-1 {
                total *= x;
            }
            return total;
        }
        _ => panic!("ERR: terminal symbol: `{}` is not an arithmetic operator.\nAllowed: [`+`, `-`, `*`, `/`, '%']", op),
    }
}

// Checks if `c` is a arithmetic operator.
fn is_sym(c: char) -> bool {
    c == '+' || c == '-' ||
        c == '*' || c == '/' ||
        c == '^'
}

fn check_priority(nums: &mut Vec<f64>, syms: &mut Vec<char>, i: usize, verbose: bool) {
    nums[i] = perform_arithmetic(nums[i], syms[i], nums[i+1], Some(nums[i+1] as i32), verbose);
    nums.remove(i+1);
    syms.remove(i);
}

// Give this function a vec of nums and symbols and it will spit out the resulting math.
fn evaluate(nums: &mut Vec<f64>, syms: &mut Vec<char>, verbose: bool) -> f64 {

    let mut total = 0.;

    // Deal with priorities, ^|*|/, left->right.
    let mut should_loop = true;
    while should_loop {
        should_loop = false;
        for i in 0..syms.len() {
            if syms[i] == '^' {
                check_priority(nums, syms, i, verbose);
                should_loop = true;
                break;
            }
        }
    }
    should_loop = true;
    while should_loop {
        should_loop = false;
        for i in 0..syms.len() {
            if syms[i] == '*' || syms[i] == '/' {
                check_priority(nums, syms, i, verbose);
                should_loop = true;
                break;
            }
        }
    }

    // Set total to the first element as a starting point.
    if nums.len() > 0 {
        total = nums[0];
    }
    if verbose {
        println!("Built: {:?} {:?}", nums, syms);
    }
    for (i, s) in syms.iter().enumerate() {
        total = perform_arithmetic(total, *s, nums[i+1], None, verbose);
        if verbose {
            println!("Done: {}", total);
        }
    }

    total
}

// Ensure that the given equation has balanced parenthesis.
fn check_paren_count(passed_eq: &str) -> bool {
    let mut count: i32 = 0;
    for c in passed_eq.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }
        if count < 0 { return false; }
    }
    count == 0
}

// Build vecs of nums and symbols.
fn parse_equation(passed_eq: &str, verbose: bool) -> f64 {

    if passed_eq.trim() == "quit" {
        println!("quiting...");
        return -0.0;
    }

    if !check_paren_count(passed_eq) {
        println!("ERR: unbalanced parenthesis.");
        return -0.0;
    }

    let mut cur_num            = String::new();
    let mut parsed_nums        = Vec::<f64>::new();
    let mut parsed_syms        = Vec::<char>::new();
    let mut paren_count: usize = 0;

    if verbose {
        println!("Parsing: {}", passed_eq);
    }

    // Loop through the passed_eq.
    for c in passed_eq.chars() {
        if c == ')' {
            paren_count -= 1;
            match paren_count {
                0 => {
                    match cur_num.len() {
                        0 => cur_num = "0".to_string(),
                        _ => cur_num = parse_equation(&cur_num.to_string(), verbose).to_string(),
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
    evaluate(&mut parsed_nums, &mut parsed_syms, verbose)
}

// Draws a graph of max-size 20 using gnuplot API. Always plots GRAPHSZ number of points.
fn draw_graph(points: Vec<Point>) {
    // const GRAPHSZ: usize = 40;
    const GRAPHSZ: usize = 10;

    if points.len() > GRAPHSZ {
        panic!("INTERNAL ERR: Something went horribly wrong.")
    }

    let mut fg                     = Figure::new();
    let mut x_vals: [f64; GRAPHSZ] = [-0.; GRAPHSZ];
    let mut y_vals: [f64; GRAPHSZ] = [-0.; GRAPHSZ];

    for i in 0..points.len() {
        x_vals[i] = points[i].x;
        y_vals[i] = points[i].y;
    }

    // gnuplot template.
    fg.axes2d()
	      .set_title("A plot", &[])
	      .set_legend(Graph(0.5), Graph(0.9), &[], &[])
	      .set_x_label("x", &[])
	      .set_y_label("y^2", &[])
	      .lines(
		        &x_vals,
		        &y_vals,
		        &[Caption("Plotted points")],
	      );

    fg.show().unwrap();
}

// Create a graph from a string. This will also call parse_equation().
fn create_graph(passed_function: &str, sz: i32, verbose: bool) -> Vec<Point> {
    let mut graph_points = Vec::<Point>::new();
    let mut new_function = String::new();
    let mut tmp: String;

    for i in -sz..sz {
        for c in passed_function.chars() {
            if c.is_alphabetic() {
                if i < 0 {
                    new_function.push('(');
                    tmp = i.to_string();
                    for c2 in tmp.chars() {
                        // Push every digit.
                        new_function.push(c2);
                    }
                    new_function.push(')');
                }
                else {
                    tmp = i.to_string();
                    for c2 in tmp.chars() {
                        // Push every digit.
                        new_function.push(c2);
                    }
                }
            }
            else {
                new_function.push(c);
            }
        }
        graph_points.push(Point::new(i as f64, parse_equation(&new_function, verbose)));
        new_function.clear();
    }
    graph_points
}

// Get information on a graph from stdin.
fn get_graph_info(graphvis: bool) -> GraphInput {
    let mut buffer          = String::new();
    let mut graph_input     = GraphInput::new("".to_string(), 0);
    const GRAPHVIS_AMT: i32 = 20;

    println!("f(x) = ? ");
    stdin().read_line(&mut graph_input.y).expect("Failed to read into buffer.");
    buffer.clear();

    if !graphvis {
        println!("Amount to test?");
        stdin().read_line(&mut buffer).expect("Failed to read into buffer.");
        graph_input.amt = buffer.trim().parse::<i32>().unwrap();
    }
    else {
        // Graphvis will always test GRAPHVIS_AMT, so there's no need to take input here.
        graph_input.amt = GRAPHVIS_AMT;
    }

    println!("Graphing...");
    graph_input
}

// Print beginning information to stdout.
fn print_begin_info(verbose: bool) {
    println!("\n\nType: `quit` to quit the program.");
    println!("Type: `graph` to print `n` number of points on a graph.");
    println!("Type: `graphvis` to print `n` number of points on a graph and draw a visual graph.");
    println!("[NOTE] `graphvis` needs gnuplot installed to function properly.");
    println!("To enable verbose mode, re-run with `-v`");
    println!("Verbose mode enabled? [{}]", verbose);
}

fn main() {
    let mut graph_input: GraphInput;
    let mut buffer      = String::new();
    let mut history     = Vec::<f64>::new();
    let args: String    = std::env::args().collect();
    let verbose: bool   = if args.trim() == "./graphing-v" { true } else { false };

    print_begin_info(verbose);

    while buffer.trim() != "quit" {
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Failed to read into buffer.");

        // Print plot points on a graph.
        if buffer.trim() == "graph" {
            graph_input = get_graph_info(false);
            println!("|\nV\n--------------------------------");
            for graph in create_graph(&graph_input.y, graph_input.amt, verbose) {
                println!("x: [{}] y:[{}]", graph.x, graph.y);
            }
            println!("--------------------------------\n");
            graph_input.y.clear();
        }

        // Draw a visual graph.
        else if buffer.trim() == "graphvis" {
            graph_input = get_graph_info(true);
            println!("|\nV\n--------------------------------\nDrawing...");
            draw_graph(create_graph(&graph_input.y, graph_input.amt, verbose));
            println!("--------------------------------\n");
            graph_input.y.clear();
        }

        // Anything else is the calculator.
        else {
            println!("|\nV\n--------------------------------");
            history.push(parse_equation(&buffer, verbose));
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
}

