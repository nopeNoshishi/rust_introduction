pub fn rpn(exp: &str) -> f64 {
    let mut stack = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num)
        } else {
            match token {
                "+" => apply(&mut stack, |x, y| x + y),
                "-" => apply(&mut stack, |x, y| x - y),
                "*" => apply(&mut stack, |x, y| x * y),
                "/" => apply(&mut stack, |x, y| x / y),
                _ => panic!("Unknow operator: {}", token),
            }
        }
    }

    stack.pop().expect("Stack underflow")
}

fn apply(stack: &mut Vec<f64>, f: impl Fn(f64, f64) -> f64) {
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = f(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpn() {
        let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

        println!("{}", rpn(exp))
    }
}
