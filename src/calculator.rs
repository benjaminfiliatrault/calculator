#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Subtract,
    Multiple,
    Divide,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Operator(Operator),
    Bracket(char),
} 

pub struct Calculator {

}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchParens,
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expression: T) -> Result<Vec<Token>, Error> {
        let formula = expression.as_ref();
        let chars = formula.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();

        for c in chars {
            match c {
                '0'..= '9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },

                '(' => {
                    tokens.push(Token::Bracket(c));
                    parens.push(c);
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchParens)
                        }
                    } else {
                        return Err(Error::MismatchParens)
                    }
                },
                
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => tokens.push(Token::Operator(Operator::Subtract)),
                '*' => tokens.push(Token::Operator(Operator::Multiple)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),
             
                ' ' => {},
                '\n' => {},
                _ => return Err(Error::BadToken(c))
            }
        }

        if parens.len() > 0 {
            return Err(Error::MismatchParens);
        }

        return Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Operator(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token && matches!(stack[stack.len() - 1], Token::Operator(_)) {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Bracket('(') => {
                    stack.push(token)
                },
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {},
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap())
        }

        return queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),

                Token::Operator(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                },
                Token::Operator(Operator::Subtract) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                },
                Token::Operator(Operator::Multiple) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                },
                Token::Operator(Operator::Divide) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                },

                _ => {},
            }
        }

        if stack.len() > 1 {
            return None;
        }
        
        return stack.pop();
    }


}