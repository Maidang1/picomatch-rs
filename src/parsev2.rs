use std::cell::RefCell;
use std::rc::Rc;
use std::{cmp::min, default, os::macos::raw::stat};

use crate::utils::{slice, HasPrefix};
#[derive(Debug, Clone)]
struct ParseOptions {
    max_length: Option<i32>,
    prepend: Option<String>,
    capture: Option<bool>,
    windows: Option<bool>,
    dot: Option<bool>,
    bash: Option<bool>,
    noext: Option<bool>,
    noextglob: Option<bool>,
    noglobstar: Option<bool>,
    strict_slashes: Option<bool>,
}

struct ParseInput {}

#[derive(Debug, Clone)]
struct Bos {
    box_type: Option<String>,
    value: Option<String>,
    output: Option<String>,
}

#[derive(Default, Clone)]
struct ParseState {
    input: String,
    index: i32,
    start: i32,
    dot: bool,
    consumed: String,
    output: String,
    prefix: Option<String>,
    backtrack: bool,
    negated: bool,
    brackets: i32,
    braces: i32,
    parens: i32,
    quotes: i32,
    globstar: bool,
    tokens: Vec<Bos>,
    peek: String,
    advance: String,
}
impl ParseState {
    // Add methods to replace the existing closures
    fn peek(&mut self, n: Option<i32>) -> String {
        let n = n.unwrap_or(1);
        let str = self
            .input
            .chars()
            .nth((self.index + n) as usize)
            .unwrap_or(' ')
            .to_string();
        self.peek = str.clone();
        str
    }

    fn advance(&mut self) -> String {
        let index = self.index + 1;
        let str = self
            .input
            .chars()
            .nth(index as usize)
            .unwrap_or(' ')
            .to_string();
        self.advance = str.clone();
        str
    }

    fn remaining(&self) -> String {
        let chars = self.input.chars().as_str();
        let idx = (self.index + 1) as usize;
        chars[idx..].to_string()
    }

    fn consume(&mut self, value: Option<String>, num: Option<i32>) {
        let value = value.unwrap_or("".to_string());
        let num = num.unwrap_or(1);
        self.consumed = format!("{}{}", self.consumed, value);
        self.index += num;
    }

    fn append(&mut self, token: &Token) {
        if token.output.is_some() {
            self.output = format!("{}{}", self.output, token.output.clone().unwrap());
        } else {
            self.output = format!("{}{}", self.output, token.value.clone().unwrap());
        }
        self.consume(Some(token.value.clone().unwrap_or("".to_string())), None);
    }

    fn eos(&self, len: i32) -> bool {
        self.index == len - 1
    }

    fn negate(&mut self) -> bool {
        let mut count = 1;

        while self.peek(None) == "!" && (self.peek(Some(2)) != "(" || self.peek(Some(3)) != "?") {
            self.advance();
            self.start += 1;
            count += 1;
        }

        if count % 2 == 0 {
            return false;
        }

        self.negated = true;
        self.start += 1;
        true
    }
}

struct Token {
    output: Option<String>,
    value: Option<String>,
    token_type: Option<String>,
    ext_glob: Option<bool>,
}

impl HasPrefixAndNegated for ParseState {
    fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    fn negated(&self) -> bool {
        self.negated
    }

    fn set_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix);
    }
}

use crate::{
    constants::*,
    utils::{extglob_chars, glob_chars, remove_prefix, HasPrefixAndNegated},
};
pub fn parse(input: &str, options: &ParseOptions) -> ParseInput {
    let mut options = options.clone();
    let mut input = REPLACEMENTS.get(input).unwrap_or(&input).to_string();
    let max = if options.max_length.is_some() {
        min(*MAX_LENGTH, options.max_length.unwrap())
    } else {
        *MAX_LENGTH
    };

    println!("input: {:?}", options);

    let mut len = input.len();

    if len > max.try_into().unwrap() {
        panic!("Input is too long. Max length is {}", max);
    }

    let prepend = options.prepend.clone();
    let bos = Bos {
        box_type: Some("bos".to_string()),
        value: Some("".to_string()),
        output: Some(prepend.unwrap_or("".to_string())),
    };

    let tokens = vec![&bos];

    let capture = if options.capture.unwrap_or(false) {
        ""
    } else {
        "?:"
    };

    let PLATFORM_CHARS = glob_chars(options.windows.unwrap_or(false));
    let EXTGLOB_CHARS = extglob_chars(&PLATFORM_CHARS);

    let globstar = |opts: &ParseOptions| {
        let dot_literal = PLATFORM_CHARS.get("DOT_LITERAL").unwrap();
        let start_anchor = PLATFORM_CHARS.get("START_ANCHOR").unwrap();
        let dots = if opts.dot.unwrap_or(false) {
            *DOTS_SLASH
        } else {
            *dot_literal
        };
        format!("({capture}(?:(?!{start_anchor}{dots}).)*?)")
    };

    let (no_dot, qmark_no_dot) = if options.dot.unwrap_or(false) {
        ((*NO_DOT).to_string(), (*QMARK_NO_DOT).to_string())
    } else {
        ("".to_string(), (*QMARK).to_string())
    };

    let mut star = if options.bash.unwrap_or(false) {
        (*STAR).to_string()
    } else {
        globstar(&options)
    };

    if let Some(capture) = options.capture {
        if capture {
            star = format!("({star})");
        }
    }
    if let Some(noext) = options.noext {
        options.noextglob = Some(noext);
    }

    let mut state = ParseState::default();

    input = remove_prefix(input.as_str(), &mut state);
    len = input.len();

    let extglobs: Vec<_> = vec!["1"];
    // let braces:Vec<_> = vec![];
    // let stack:Vec<_> = vec![];

    let mut prev = &bos;
    let value = String::from("");
    // const increment = type => {
    //   state[type]++;
    //   stack.push(type);
    // };

    // const decrement = type => {
    //   state[type]--;
    //   stack.pop();
    // };
    let push = |token: &Token, prev: &mut Bos| {
        if prev.box_type.as_ref().unwrap() == &String::from("globstar") {
            let token_type = token.token_type.clone().unwrap();
            let is_brace = state.braces > 0
                && (token_type == String::from("comma") || token_type == String::from("brace"));
            let is_extglob = token.ext_glob == Some(true)
                || (extglobs.len() > 0
                    && (token_type == String::from("pipe") || token_type == String::from("paren")));
            if token_type != String::from("slash")
                && token_type != String::from("paren")
                && !is_brace
                && !is_extglob
            {
                let pre_len = state.output.len() as i32;
                state.output = slice(
                    &state.output.as_str(),
                    0,
                    Some((pre_len * -1).try_into().unwrap()),
                );
                prev.box_type = Some(String::from("star"));
                prev.value = Some(String::from("*"));
                prev.output = Some(star.clone());
                state.output = format!("{}{}", state.output, star.clone());
            }
        }
    };

    ParseInput {}
}

#[derive(Debug, Default)]
struct FastPathState {
    negated: bool,
    prefix: String,
}

impl HasPrefix for FastPathState {
    fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }
    fn has_prefix(&self, prefix: &str) -> &String {
        &self.prefix
    }
}

pub fn fast_paths(input: &String, options: &ParseOptions) -> Option<String> {
    let mut max = if options.max_length.is_some() {
        min(*MAX_LENGTH, options.max_length.unwrap())
    } else {
        *MAX_LENGTH
    };

    let mut len = input.len() as i32;
    if len > max {
        panic!("Input is too long. Max length is {}", max);
    }

    let mut input: String = input.clone();
    if let Some(initial_input) = REPLACEMENTS.get(Box::leak(input.clone().into_boxed_str())) {
        input = initial_input.to_string()
    };

    let glob_chars = glob_chars(options.windows.unwrap_or(false));
    let one_char = glob_chars.get("ONE_CHAR").unwrap().to_string();
    let dot_literal = glob_chars.get("DOT_LITERAL").unwrap().to_string();
    let slash_literal = glob_chars.get("SLASH_LITERAL").unwrap().to_string();
    let slash_literal_clone = slash_literal.clone();

    let no_dot = if options.dot.unwrap_or(false) {
        glob_chars.get("NO_DOTS").unwrap().to_string()
    } else {
        glob_chars.get("NO_DOT").unwrap().to_string()
    };

    let slash_dot = if options.dot.unwrap_or(false) {
        glob_chars.get("NO_DOTS_SLASH").unwrap().to_string()
    } else {
        glob_chars.get("NO_DOT").unwrap().to_string()
    };

    let capture = if options.capture.unwrap_or(false) {
        "".to_string()
    } else {
        "?:".to_string()
    };

    let mut state = FastPathState::default();

    let mut star = if options.bash.unwrap_or(false) {
        ".*?".to_string()
    } else {
        glob_chars.get("STAR").unwrap().to_string()
    };

    if options.capture.unwrap_or(false) {
        star = format!("({})", star);
    }

    let glob_star = |opts: &ParseOptions| {
        if opts.noglobstar == Some(true) {
            return format!("{}", star);
        } else {
            let dot = if opts.dot.unwrap_or(false) {
                glob_chars.get("DOTS_SLASH").unwrap().to_string()
            } else {
                glob_chars.get("DOT_LITERAL").unwrap().to_string()
            };
            return format!("({}(?:(?!{}{}).)*?)", capture, *START_ANCHOR, dot);
        }
    };

    let create: Rc<RefCell<Option<Box<dyn Fn(&String, &String) -> Option<String>>>>> =
        Rc::new(RefCell::new(None));

    let create_clone = create.clone();
    *create_clone.borrow_mut() = Some(Box::new(
        move |str: &String, star: &String| -> Option<String> {
            let str = str.as_str();

            let result = match str {
                "*" => {
                    format!("{}{}{}", no_dot, one_char, star)
                }
                ".*" => {
                    format!("{}{}{}", dot_literal, one_char, star)
                }
                "*.*" => {
                    format!("{}{}{}{}{}", no_dot, star, dot_literal, one_char, star)
                }
                "*/*" => {
                    format!(
                        "{}{}{}{}{}{}",
                        no_dot, star, slash_literal, one_char, slash_dot, star
                    )
                }
                "**" => {
                    format!("{}{}", no_dot, glob_star(&options))
                }
                "**/*" => {
                    format!(
                        "(?:{}{}{}{}{}{}",
                        no_dot,
                        glob_star(&options),
                        slash_literal,
                        slash_dot,
                        one_char,
                        star
                    )
                }
                "**/.*" => {
                    format!(
                        "(?:{}{}{}?{}{}{}",
                        no_dot,
                        glob_star(&options),
                        slash_literal,
                        dot_literal,
                        one_char,
                        star
                    )
                }
                _ => {
                    let re = Regex::new(r"^(.*?)\.(\w+)$").unwrap();
                    if !re.is_match(str) {
                        return None;
                    }
                    let result = re.captures(str).unwrap();
                    let match1 = result.get(1).map(|m| m.as_str()).unwrap();
                    let match2 = result.get(2).map(|m| m.as_str()).unwrap();
                    let source = create.borrow().as_ref().unwrap()(&match1.to_string(), star);
                    if source.is_none() {
                        return None;
                    }
                    format!("{}{}{}", source.unwrap(), *DOT_LITERAL, match2.to_string())
                }
            };

            Some(result)
        },
    ));

    // let output = remove_prefix(input.clone().as_str(), &mut state).to_string();
    // let source = create_clone.borrow().as_ref().unwrap()(&output.clone(), &star.clone());

    // if source.is_some() && options.strict_slashes != Some(true) {
    //     return Some(format!("{}{}?", source.unwrap(), slash_literal_clone));
    // }
    // source
}
