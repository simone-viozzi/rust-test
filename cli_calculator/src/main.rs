use clap::Parser;

/// A simple CLI calculator program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The first number
    #[arg(long)]
    num1: f64,

    /// The second number
    #[arg(long)]
    num2: f64,

    /// The operation to perform: add, sub, mul, div
    #[arg(short, long)]
    operation: String,
}

fn main() {
    let args = Args::parse();

    let result = match args.operation.as_str() {
        "add" => args.num1 + args.num2,
        "sub" => args.num1 - args.num2,
        "mul" => args.num1 * args.num2,
        "div" => {
            if args.num2 != 0.0 {
                args.num1 / args.num2
            } else {
                eprintln!("Error: Division by zero");
                return;
            }
        }
        _ => {
            eprintln!("Error: Unsupported operation '{}'", args.operation);
            return;
        }
    };

    println!("Result: {}", result);
}
