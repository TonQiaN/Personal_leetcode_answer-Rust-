struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut my_stack = vec![];
        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                my_stack.push(num);
            } else {
                let rhs = my_stack.pop().unwrap();
                let lhs = my_stack.pop().unwrap();
                match &token[..] {
                    "+" => my_stack.push(lhs + rhs),
                    "-" => my_stack.push(lhs - rhs),
                    "*" => my_stack.push(lhs * rhs),
                    "/" => my_stack.push(lhs / rhs),
                    _ => {},
                }
            }
            // match &token[..] {
            //     "+" => {
            //         let a = my_stack.pop().unwrap();
            //         let b = my_stack.pop().unwrap();
            //         my_stack.push(a + b);
            //     },
            //     "-" => {
            //         let a = my_stack.pop().unwrap();
            //         let b = my_stack.pop().unwrap();
            //         my_stack.push(b - a);
            //     },
            //     "*" => {
            //         let a = my_stack.pop().unwrap();
            //         let b = my_stack.pop().unwrap();
            //         my_stack.push(a * b);
            //     },
            //     "/" => {
            //         let a = my_stack.pop().unwrap();
            //         let b = my_stack.pop().unwrap();
            //         my_stack.push(b / a);
            //     },
            //     number => {
            //         let num: i32 = number.parse().unwrap();
            //         my_stack.push(num);
            //     },
            // }
        }
        my_stack[0]        
    }
}

fn main() {
    println!("Hello, world!");
}
