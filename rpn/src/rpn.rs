use anyhow::{Result, bail};

pub trait ReversePolishNotation {
    fn calculate_rpn(&self) -> Result<f64>;
}

impl ReversePolishNotation for str {
    fn calculate_rpn(&self) -> Result<f64> {
        let mut stack = Vec::new();

        for token in self.split_whitespace() {
            if let Ok(num) = token.parse::<f64>() {
                stack.push(num)
            } else {
                match token {
                    "+" => apply(&mut stack, |x, y| x + y)?,
                    "-" => apply(&mut stack, |x, y| x - y)?,
                    "*" => apply(&mut stack, |x, y| x * y)?,
                    "/" => apply(&mut stack, |x, y| x / y)?,
                    _ => panic!("Unknow operator: {}", token),
                }
            }
        }
    
        Ok(stack.pop().unwrap())
    }
}

fn apply(stack: &mut Vec<f64>, f: impl Fn(f64, f64) -> f64) -> Result<()> {
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = f(x, y);
        stack.push(z);
    } else {
        bail!("Cant aaply notaion")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpn() {
        let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -"; 

        let result = exp.calculate_rpn();

        assert!(result.is_ok());
        assert_eq!(26.284000000000002, result.unwrap());

        let exp = "6.1 5.2 * + 3.4 2.5 / 1.6 * -"; 

        let result = exp.calculate_rpn();

        assert!(result.is_err());

    }
}