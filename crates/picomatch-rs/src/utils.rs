use crate::constants::{CHAR_BACKWARD_SLASH, CHAR_FORWARD_SLASH};

pub fn is_path_separator(ch: char) -> bool {
    ch == CHAR_FORWARD_SLASH || ch == CHAR_BACKWARD_SLASH
}

pub fn remove_backslashes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '[' {
            result.push(ch);

            while let Some(current) = chars.next() {
                result.push(current);

                if current == CHAR_BACKWARD_SLASH {
                    if let Some(escaped) = chars.next() {
                        result.push(escaped);
                    }
                } else if current == ']' {
                    break;
                }
            }

            continue;
        }

        if ch == CHAR_BACKWARD_SLASH && chars.peek() == Some(&'[') {
            continue;
        }

        if ch == CHAR_BACKWARD_SLASH {
            if let Some(next) = chars.next() {
                result.push(next);
            }
            continue;
        }

        result.push(ch);
    }

    result
}
