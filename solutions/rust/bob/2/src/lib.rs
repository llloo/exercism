pub fn reply(message: &str) -> &str {
    let s = message.trim();

    if s.is_empty() {
        return "Fine. Be that way!";
    }

    let is_yelling = s.chars().any(|c| c.is_alphabetic()) && s == s.to_uppercase();
    let is_question = s.ends_with("?");

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}
