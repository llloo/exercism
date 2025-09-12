use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation = parse_equation(input)?;
    let mut solver = AlphameticsSolver::new(equation);
    solver.solve()
}

struct Equation {
    left: Vec<Vec<char>>,
    right: Vec<char>,
    first_chars: HashSet<char>,
    all_chars: Vec<char>,
}

struct AlphameticsSolver {
    equation: Equation,
    leading_zeros: HashSet<char>,
}

impl AlphameticsSolver {
    fn new(equation: Equation) -> Self {
        let leading_zeros = equation.first_chars.clone();
        Self {
            equation,
            leading_zeros,
        }
    }

    fn solve(&mut self) -> Option<HashMap<char, u8>> {
        let mut assignment = HashMap::new();
        let mut used_digits = [false; 10];
        self.backtrack(0, &mut assignment, &mut used_digits)
    }

    fn backtrack(
        &self,
        index: usize,
        assignment: &mut HashMap<char, u8>,
        used_digits: &mut [bool; 10],
    ) -> Option<HashMap<char, u8>> {
        if index == self.equation.all_chars.len() {
            return if self.is_valid(assignment) {
                Some(assignment.clone())
            } else {
                None
            };
        }

        let current_char = self.equation.all_chars[index];
        let start_digit = if self.leading_zeros.contains(&current_char) {
            1
        } else {
            0
        };

        for digit in start_digit..10 {
            if used_digits[digit as usize] {
                continue;
            }

            assignment.insert(current_char, digit);
            used_digits[digit as usize] = true;

            if let Some(solution) = self.backtrack(index + 1, assignment, used_digits) {
                return Some(solution);
            }

            assignment.remove(&current_char);
            used_digits[digit as usize] = false;
        }

        None
    }

    fn is_valid(&self, assignment: &HashMap<char, u8>) -> bool {
        let left_sum: u64 = self
            .equation
            .left
            .iter()
            .map(|word| word_to_number(word, assignment))
            .sum();

        let right_sum = word_to_number(&self.equation.right, assignment);

        left_sum == right_sum
    }
}

fn word_to_number(word: &[char], assignment: &HashMap<char, u8>) -> u64 {
    word.iter()
        .fold(0, |acc, &c| acc * 10 + *assignment.get(&c).unwrap() as u64)
}

fn parse_equation(input: &str) -> Option<Equation> {
    let parts: Vec<&str> = input.split(" == ").collect();
    if parts.len() != 2 {
        return None;
    }

    let left_side: Vec<&str> = parts[0].split(" + ").collect();
    let right_side = parts[1];

    let left_words: Vec<Vec<char>> = left_side.iter().map(|s| s.chars().collect()).collect();

    let right_word: Vec<char> = right_side.chars().collect();

    // 收集所有字符和首字符
    let mut all_chars = HashSet::new();
    let mut first_chars = HashSet::new();

    for word in &left_words {
        if let Some(&first) = word.first() {
            first_chars.insert(first);
        }
        for &c in word {
            all_chars.insert(c);
        }
    }

    if let Some(&first) = right_word.first() {
        first_chars.insert(first);
    }
    for &c in &right_word {
        all_chars.insert(c);
    }

    let all_chars_vec: Vec<char> = all_chars.into_iter().collect();

    Some(Equation {
        left: left_words,
        right: right_word,
        first_chars,
        all_chars: all_chars_vec,
    })
}
