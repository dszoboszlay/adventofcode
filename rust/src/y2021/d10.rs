use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let mut score: i64 = 0;
    let mut stack: Vec<char> = Vec::new();
    
    input.chars().fold(true, |valid, c| {
        match c {
            '\n' => {
                stack.clear();
                true
            },
            _ if !valid => false,
            '(' => {
                stack.push(')');
                true
            },
            '[' => {
                stack.push(']');
                true
            },
            '{' => {
                stack.push('}');
                true
            },
            '<' => {
                stack.push('>');
                true
            },
            ')' => {
                (stack.pop() == Some(c)) || {
                    score += 3;
                    false
                }
            },
            ']' => {
                (stack.pop() == Some(c)) || {
                    score += 57;
                    false
                }
            },
            '}' => {
                (stack.pop() == Some(c)) || {
                    score += 1197;
                    false
                }
            },
            '>' => {
                (stack.pop() == Some(c)) || {
                    score += 25137;
                    false
                }
            },
            _ => panic!("invalid input file")
        }
    });
    
    score.to_string()
}

pub fn p02(input: &String) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut scores: Vec<i64> = Vec::new();
    
    input.chars().fold(true, |valid, c| {
        match c {
            '\n' => {
                if valid {
                    scores.push(stack.iter().rfold(0 as i64, |sum, c| {
                        sum * 5 + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!("invalid stack")
                        }
                    }));
                }
                stack.clear();
                true
            },
            _ if !valid => false,
            '(' => {
                stack.push(')');
                true
            },
            '[' => {
                stack.push(']');
                true
            },
            '{' => {
                stack.push('}');
                true
            },
            '<' => {
                stack.push('>');
                true
            },
            ')' | ']' | '}' | '>' => stack.pop() == Some(c),
            _ => panic!("invalid input file")
        }
    });
    
    scores.sort();
    scores[scores.len() / 2].to_string()
}
