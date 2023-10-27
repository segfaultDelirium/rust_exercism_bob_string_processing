fn is_question(message: &str) -> bool {
    message.chars().last().unwrap() == '?'
}

fn is_yelling(message: &str) -> bool {
    if message
        .to_lowercase()
        .chars()
        .filter(|c| ('a'..='z').contains(c))
        .collect::<Vec<char>>()
        .is_empty()
    {
        return false;
    }
    message.chars().all(|c| match c {
        'a'..='z' => false,
        'A'..='Z' => true,
        _ => true,
    })
}

pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();
    if trimmed_message.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = is_question(trimmed_message);
    let is_yelling = is_yelling(trimmed_message);
    if is_yelling && is_question {
        return "Calm down, I know what I'm doing!";
    }
    if is_question {
        return "Sure.";
    }
    if is_yelling {
        return "Whoa, chill out!";
    }
    // return "Sure.";

    "Whatever."
}
