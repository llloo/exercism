pub fn reply(message: &str) -> &str {
    let s = message.trim();

    if s.is_empty() {
        return "Fine. Be that way!";
    }

    if s.ends_with("?") && s == s.to_uppercase() && s.chars().any(|c| c.is_uppercase()) {
        return "Calm down, I know what I'm doing!";
    }

    if s.ends_with("?") {
        return "Sure.";
    }

    if s == s.to_uppercase() && s.chars().any(|c| c.is_alphabetic()) {
        return "Whoa, chill out!";
    }

    "Whatever."
}
