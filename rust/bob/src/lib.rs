pub fn reply(message: &str) -> &str {
    if message.chars().all(|c| c.is_whitespace()) || message.is_empty() {
        return "Fine. Be that way!"
    }

    let message = message.trim_end();
    let is_question = message.ends_with("?");
    let is_all_capital = message.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) && message.chars().any(|c| c.is_alphabetic());
    match (is_question, is_all_capital) {
        (true, false) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        (false, false) => "Whatever.",
        (false, true) => "Whoa, chill out!",
    }
}
