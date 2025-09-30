use microlisp::{Environment, Error, Expression};

fn main() -> Result<(), Error> {
    let mut env = Environment::new();
    env.load_default_builtins()?;

    println!("=== Basic Microlisp Example ===\n");

    // Parse and evaluate a simple expression
    let result = env.parse_eval("(+ 1 2 3)")?;
    println!("(+ 1 2 3) = {}", result);
    assert_eq!(result, Expression::Number(6));

    // Define variables
    env.parse_eval("(def x 42)")?;
    let result = env.parse_eval("(* x 2)")?;
    println!("After (def x 42): (* x 2) = {}", result);
    assert_eq!(result, Expression::Number(84));

    // Local bindings with let
    let result = env.parse_eval("(let [a 10 b 20] (+ a b))")?;
    println!("(let [a 10 b 20] (+ a b)) = {}", result);
    assert_eq!(result, Expression::Number(30));

    // Conditional expressions
    let result = env.parse_eval("(if (> x 41) true false)")?;
    println!("(if (> x 41) true false) = {}", result);
    assert_eq!(result, Expression::Bool(true));

    // Sequential execution with 'do'
    let result = env.parse_eval("(do (def temp 5) (+ temp 10) (* temp 3))")?;
    println!("(do (def temp 5) (+ temp 10) (* temp 3)) = {}", result);
    assert_eq!(result, Expression::Number(15));

    // Loops with dotimes
    env.parse_eval("(def sum 0)")?;
    env.parse_eval("(dotimes [i 5] (def sum (+ sum i)))")?;
    let result = env.parse_eval("(+ sum 0)")?; // Note: direct variable access needs expression context
    println!("(dotimes [i 5] (def sum (+ sum i))) = {}", result);
    assert_eq!(result, Expression::Number(10));

    println!("\n=== Example completed successfully! ===");
    Ok(())
}

