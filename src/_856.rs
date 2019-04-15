pub struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    LeftP,
    RightP,
    Score(i32),
}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack: Vec<State> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => Self::add_to_stack(&mut stack, State::LeftP),
                ')' => Self::add_to_stack(&mut stack, State::RightP),
                _ => unreachable!(),
            }
        }
        stack.last().cloned().map_or(0, |x| match x {
            State::Score(value) => value,
            _ => unreachable!(),
        })
    }

    fn add_to_stack(stack: &mut Vec<State>, state: State) {
        match state {
            State::LeftP => stack.push(State::LeftP),
            State::RightP => match stack.last().cloned().unwrap() {
                State::LeftP => {
                    stack.pop();
                    Self::add_to_stack(stack, State::Score(1));
                }
                State::Score(value) => {
                    stack.pop();
                    debug_assert_eq!(stack.last(), Some(&State::LeftP));
                    stack.pop();
                    Self::add_to_stack(stack, State::Score(value * 2));
                }
                State::RightP => unreachable!(),
            },
            State::Score(value) => match stack.last().cloned() {
                Some(x) => match x {
                    State::LeftP => stack.push(State::Score(value)),
                    State::Score(value2) => {
                        stack.pop();
                        Self::add_to_stack(stack, State::Score(value + value2));
                    }
                    State::RightP => unreachable!(),
                },
                None => {
                    stack.push(State::Score(value));
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
    }
}
