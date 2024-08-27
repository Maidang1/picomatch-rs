use crate::{constants::*, utils::remove_back_slashes};

#[derive(Debug, PartialEq)]
struct ScanState {
    prefix: String,
    input: String,
    start: i32,
    base: String,
    glob: String,
    is_brace: bool,
    is_glob: bool,
    is_ext_glob: bool,
    is_glob_star: bool,
    negated: bool,
    negated_extglob: bool,
    is_bracket: bool,
    max_depth: Option<i32>,
    tokens: Option<Vec<Token>>,
}

#[derive(Default)]
struct ScanOptions {
    pub parts: Option<bool>,
    pub scan_to_end: Option<bool>,
    pub noext: Option<bool>,
    pub nonegate: Option<bool>,
    pub noparen: Option<bool>,
    pub unescape: Option<bool>,
    pub tokens: Option<bool>,
}

#[derive(Default, Clone, Debug, PartialEq)]
struct Token {
    pub value: String,
    pub depth: i32,
    pub is_glob: bool,
    pub back_slashes: bool,
    pub is_brace: bool,
    pub is_ext_glob: bool,
    pub is_glob_star: bool,
    pub is_bracket: bool,
    pub is_prefix: Option<bool>,
    pub negated: Option<bool>,
}

struct Part {}

struct Scan {}

fn is_path_separator(code: &i32) -> bool {
    return *code == *CHAR_FORWARD_SLASH || *code == *CHAR_BACKWARD_SLASH;
}

impl Scan {
    pub fn new() -> Scan {
        Scan {}
    }
}

impl Scan {
    pub fn scan(&self, input: String, options: ScanOptions) -> ScanState {
        let length = input.len() as i32;
        let length = length - 1;
        let scan_to_end = options.scan_to_end.unwrap_or(false);
        let parts = options.parts.unwrap_or(false);
        let scan_to_end = scan_to_end || parts;

        let mut slashes: Vec<i32> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        // let mut parts: Vec<Part> = Vec::new();

        let mut str = input.clone();
        let mut index = -1 as i32;
        let mut start = 0;
        let mut last_index = 0;
        let mut is_brace = false;
        let mut is_bracket = false;
        let mut is_glob = false;
        let mut is_ext_glob = false;
        let mut is_glob_star = false;
        let mut brace_escaped = false;
        let mut back_slashes = false;
        let mut negated = false;
        let mut negated_extglob = false;
        let mut finished = false;
        let mut braces = 0;
        let mut prev: i32 = -1;
        let mut code: i32 = -1;
        let mut token = Token::default();

        let eos = |index: &i32, length: &i32| index >= length;
        let peek = |index: &i32| {
            str.chars()
                .nth((index + 1).try_into().unwrap())
                .map(|c| c as i32)
                .unwrap()
        };
        let advance = |index: &mut i32, code: &mut i32, prev: &mut i32| -> Option<u32> {
            *prev = *code;
            *index += 1;
            str.chars().nth(*index as usize).map(|c| c as u32)
        };

        while index < length {
            code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
            let mut next: i32;

            if code == *CHAR_BACKWARD_SLASH {
                back_slashes = true;
                token.back_slashes = true;
                code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                if code == *CHAR_LEFT_CURLY_BRACE {
                    brace_escaped = true;
                }
            }

            if brace_escaped == true || code == *CHAR_LEFT_CURLY_BRACE {
                braces = braces + 1;

                while eos(&index, &length) != true {
                    code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                    if code == *CHAR_BACKWARD_SLASH {
                        token.back_slashes = true;
                        back_slashes = true;
                        advance(&mut index, &mut code, &mut prev);
                        continue;
                    }

                    if code == *CHAR_LEFT_CURLY_BRACE {
                        braces = braces + 1;
                        continue;
                    }

                    if brace_escaped != true && code == *CHAR_DOT {
                        code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                        if code == *CHAR_DOT {
                            is_brace = true;
                            token.is_brace = true;
                            is_glob = true;
                            token.is_glob = true;
                            finished = true;

                            if scan_to_end {
                                continue;
                            }
                            break;
                        }
                    }

                    if brace_escaped != true && code == *CHAR_COMMA {
                        token.is_brace = true;
                        is_brace = true;
                        is_glob = true;
                        token.is_glob = true;
                        finished = true;

                        if scan_to_end {
                            continue;
                        }
                        break;
                    }

                    if code == *CHAR_RIGHT_CURLY_BRACE {
                        braces = braces - 1;
                        if braces == 0 {
                            brace_escaped = false;
                            is_brace = true;
                            token.is_brace = true;
                            finished = true;
                            break;
                        }
                    }
                }

                if scan_to_end {
                    continue;
                }
                break;
            }

            if code == *CHAR_FORWARD_SLASH {
                slashes.push(index.clone());
                tokens.push(token.clone());
                token = Token::default();
                if finished {
                    continue;
                }
                if prev == *CHAR_DOT {
                    if index == start + 1 {
                        start = start + 2;
                        continue;
                    }
                }

                last_index = index + 1;
                continue;
            }

            if options.noext != Some(true) {
                let is_ext_glob_char = code == *CHAR_PLUS
                    || code == *CHAR_AT
                    || code == *CHAR_ASTERISK
                    || code == *CHAR_QUESTION_MARK
                    || code == *CHAR_EXCLAMATION_MARK;
                if is_ext_glob_char && peek(&index) == *CHAR_LEFT_PARENTHESES {
                    is_glob = true;
                    token.is_glob = true;
                    is_ext_glob = true;
                    token.is_ext_glob = true;
                    finished = true;

                    if code == *CHAR_EXCLAMATION_MARK && index == start {
                        negated_extglob = true
                    }

                    if scan_to_end {
                        while eos(&index, &length) != true {
                            code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                            if code == *CHAR_BACKWARD_SLASH {
                                back_slashes = true;
                                token.back_slashes = true;
                                code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                                continue;
                            }

                            if code == *CHAR_RIGHT_PARENTHESES {
                                is_glob = true;
                                token.is_glob = true;
                                finished = true;
                                break;
                            }
                        }
                        continue;
                    }
                    break;
                }
            }

            if code == *CHAR_ASTERISK {
                if prev == *CHAR_ASTERISK {
                    is_glob_star = true;
                    token.is_glob_star = true;
                }
                is_glob = true;
                token.is_glob = true;

                if scan_to_end {
                    continue;
                }
                break;
            }

            if code == *CHAR_QUESTION_MARK {
                is_glob = true;
                token.is_glob = true;

                if scan_to_end {
                    continue;
                }
                break;
            }

            if code == *CHAR_LEFT_SQUARE_BRACKET {
                while eos(&index, &length) != true {
                    next = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                    if next == *CHAR_BACKWARD_SLASH {
                        back_slashes = true;
                        token.back_slashes = true;
                        advance(&mut index, &mut code, &mut prev);
                        continue;
                    }

                    if next == *CHAR_RIGHT_SQUARE_BRACKET {
                        is_bracket = true;
                        token.is_bracket = true;
                        is_glob = true;
                        token.is_glob = true;
                        finished = true;
                        break;
                    }
                }

                if scan_to_end {
                    continue;
                }
                break;
            }

            if options.nonegate != Some(true) && code == *CHAR_EXCLAMATION_MARK && index == start {
                negated = true;
                token.negated = Some(true);
                start = start + 1;
                continue;
            }

            if options.noparen != Some(true) && code == *CHAR_LEFT_PARENTHESES {
                is_glob = true;
                token.is_glob = true;

                if scan_to_end {
                    while eos(&index, &length) != true {
                        code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;

                        if code == *CHAR_LEFT_PARENTHESES {
                            back_slashes = true;
                            token.back_slashes = true;
                            code = advance(&mut index, &mut code, &mut prev).unwrap() as i32;
                            continue;
                        }

                        if code == *CHAR_RIGHT_PARENTHESES {
                            finished = true;
                            break;
                        }
                    }
                    continue;
                }
                break;
            }

            if is_glob {
                finished = true;
                if scan_to_end {
                    continue;
                }
                break;
            }
        }

        if options.noext == Some(true) {
            is_ext_glob = false;
            is_glob = false;
        }

        let mut base = str.clone();
        let mut prefix = String::new();
        let mut glob = String::new();

        if start > 0 {
            prefix = str.clone()[0..start as usize].to_string();
            str = str.clone()[start as usize..].to_string();
            last_index = last_index - start;
        }

        if base.len() > 0 && is_glob && last_index > 0 {
            base = str.clone()[0..last_index as usize].to_string();
            glob = str.clone()[last_index as usize..].to_string();
        } else if is_glob {
            base = "".to_string();
            glob = str.clone();
        } else {
            base = str.clone();
        }

        if base.len() > 0 && base != "/" && base != str {
            let tem_base = base.clone();
            let last_char = tem_base.chars().last().unwrap() as i32;
            if is_path_separator(&last_char) {
                base = base.clone()[0..base.len() - 1].to_string();
            }
        }

        if options.unescape == Some(true) {
            if glob.len() > 0 {
                glob = remove_back_slashes(&glob);
            }
            if base.len() > 0 && back_slashes {
                base = remove_back_slashes(&base);
            }
        }

        let mut state = ScanState {
            prefix,
            input,
            start,
            glob,
            base,
            is_brace,
            is_bracket,
            is_glob,
            is_ext_glob,
            is_glob_star,
            negated,
            negated_extglob,
            max_depth: None,
            tokens: None,
        };

        if options.tokens == Some(true) {
            state.max_depth = Some(0);
            if !is_path_separator(&code) {
                tokens.push(token.clone());
            }
            state.tokens = Some(tokens);
        }
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    include!("../fixtures/scan.test.rs");
}
