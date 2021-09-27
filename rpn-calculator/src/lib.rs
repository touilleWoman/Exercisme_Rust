#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn solve(op: &CalculatorInput, nb1: i32, nb2: i32) -> i32 {
    match op {
        CalculatorInput::Add => nb1 + nb2,
        CalculatorInput::Subtract => nb2 - nb1,
        CalculatorInput::Multiply => nb1 * nb2,
        CalculatorInput::Divide => nb2 / nb1,
        _ => panic!("Error"),
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }
    let mut stack: Vec<i32> = Vec::new();
    for x in inputs {
        if let CalculatorInput::Value(nb) = x {
            stack.push(*nb);
        } else {
            if stack.len() < 2 {
                return None;
            } else {
                let ret = solve(x, stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(ret);
            }
        }
    }
    if stack.len() == 1 {
        return stack.pop();
    } else {
        return None;
    }
}
