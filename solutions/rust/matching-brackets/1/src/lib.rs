pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut l: Vec<char> = Vec::new();
    for c in string.chars() {
        if c == '(' || c == '[' || c == '{' {
            l.push(c);
            continue;
        }

        if c == ')' || c == ']' || c == '}' {
            if l.is_empty() {
                return false;
            }

            let last_char = l.last().unwrap();
            if (c == ')' && *last_char == '(')
                || (c == ']' && *last_char == '[')
                || (c == '}' && *last_char == '{')
            {
                l.pop();
                continue;
            } else {
                return false;
            }
        }
    }

    l.is_empty()
}
