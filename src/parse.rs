use crate::utils::remove_prefix;
use crate::{
    constants::{self, *},
    utils::{extglob_chars, glob_chars},
};
use std::cell::{RefCell, RefMut};
use std::cmp::min;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct ParseOptions {
    max_length: Option<i32>,
    prepend: Option<String>,
    windows: bool,
    capture: Option<bool>,
    dot: Option<bool>,
    bash: Option<bool>,
    noext: Option<bool>,
    noextglob: Option<bool>,
}

struct Bos {
    box_type: Option<String>,
    value: Option<String>,
    output: Option<String>,
}

struct GlobStarOptions {
    dot: bool,
}
type PeekFn = fn(Option<i32>, i32) -> char;

pub struct ParseState {
    input: String,
    index: i32,
    start: i32,
    dot: bool,
    consumed: String,
    output: String,
    pub prefix: String,
    backtrack: bool,
    negated: bool,
    brackets: i32,
    braces: i32,
    parens: i32,
    quotes: i32,
    globstar: bool,
    tokens: Vec<Bos>,
}

struct ParseToken {
    output: Option<String>,
    value: Option<String>,
}

fn parse(input: String, mut options: ParseOptions) {
    let static_input = Box::leak(input.clone().into_boxed_str());
    let mut input = input.clone();
    if let Some(inner_input) = REPLACEMENTS.get(static_input) {
        input = inner_input.to_string()
    }

    let max = if options.max_length.is_some() {
        min(*MAX_LENGTH, options.max_length.unwrap())
    } else {
        *MAX_LENGTH
    };

    let mut len = input.len() as i32;

    if len > max {
        panic!("Input is too long. Max length is {}", max);
    }

    let bos = Bos {
        box_type: Some("bos".to_string()),
        value: Some("".to_string()),
        output: Some(options.prepend.unwrap_or("".to_string())),
    };

    let tokens = vec![bos];

    let capture = if options.capture.unwrap_or(false) {
        ""
    } else {
        "?:"
    };

    let PLATFORM_CHARS = glob_chars(options.windows);

    let EXTGLOB_CHARS = extglob_chars(&PLATFORM_CHARS);
    let use_dot = options.dot.unwrap_or(false);

    let glob_star = |options: GlobStarOptions| {
        let extra = if options.dot {
            PLATFORM_CHARS.get("DOTS_SLASH").unwrap().to_string()
        } else {
            PLATFORM_CHARS.get("DOT_LITERAL").unwrap().to_string()
        };
        let result = format!(
            "({}(?:(?!{}{}).)*?)",
            capture,
            PLATFORM_CHARS.get("START_ANCHOR").unwrap().to_string(),
            extra
        );
        result
    };

    let no_dot = if use_dot {
        "".to_string()
    } else {
        PLATFORM_CHARS.get("NO_DOT").unwrap().to_string()
    };
    let qmark_no_dot = if use_dot {
        PLATFORM_CHARS.get("QMARK").unwrap().to_string()
    } else {
        PLATFORM_CHARS.get("QMARK_NO_DOT").unwrap().to_string()
    };

    let bash = options.bash.unwrap_or(false);

    let mut star = if bash {
        let result = glob_star(GlobStarOptions { dot: use_dot });
        result
    } else {
        PLATFORM_CHARS.get("STAR").unwrap().to_string()
    };

    if options.capture.unwrap_or(false) {
        star = format!("({})", star);
    }

    if options.noext.is_some() {
        options.noextglob = Some(options.noext.unwrap())
    }

    let mut state = ParseState {
        input: input.clone(),
        index: 0,
        start: 0,
        dot: use_dot,
        consumed: "".to_string(),
        output: "".to_string(),
        prefix: "".to_string(),
        backtrack: false,
        negated: false,
        brackets: 0,
        braces: 0,
        parens: 0,
        quotes: 0,
        globstar: false,
        tokens,
    };

    let mut state = Rc::new(RefCell::new(state));
    let mut state_ref = state.borrow_mut();

    input = remove_prefix(&input, &mut state_ref).to_string();
    len = input.len() as i32;

    let mut ext_globa: Vec<_> = vec!["1"];
    let mut braces: Vec<_> = vec!["1"];
    let mut stack: Vec<_> = vec!["1"];

    let prev = Rc::clone(&state);
    let value = String::new();

    let eos = || {
        state_ref.index == len - 1;
    };

    let mut peek = |state: &RefMut<'_, ParseState>, n: Option<i32>| {
        let n = n.unwrap_or(1);
        let index = state.index;
        input
            .chars()
            .nth(index as usize + n as usize)
            .unwrap()
            .to_string()
    };

    let advance = |state: &mut RefMut<'_, ParseState>| {
        state.index = state.index + 1;
        let len = state.input.len() as i32;
        if state.index < len {
            return input.chars().nth(state.index as usize).unwrap().to_string();
        } else {
            return "".to_string();
        }
    };

    let mut consume =
        |state: &mut RefMut<'_, ParseState>, value: Option<String>, num: Option<i32>| {
            let num = num.unwrap_or(1);
            let value = value.unwrap_or("".to_string());
            state.consumed = format!("{}{}", state.consumed, value);
            state.index = state.index + num;
        };

    let append = |state: &mut RefMut<'_, ParseState>, token: &mut ParseToken| {
        let mut output = "".to_string();
        if token.output.is_some() {
            output = format!("{}", token.output.take().unwrap());
        } else {
            let value = format!("{}", token.value.take().unwrap_or("".to_string()));
            output = value.clone();
        }
        state.output = format!("{}{}", state_ref.output, output);
        consume(state, Some(value), None);
    };

    let negate = |state: &mut RefMut<'_, ParseState>| -> bool {
        let mut count = 1;
        while peek(&state_ref, None) == '!'.to_string()
            && (peek(&state_ref, Some(2)) != "(" || peek(&state_ref, Some(3)) == "?")
        {
            advance(&mut state_ref);
            count = count + 1;
            state_ref.start = state_ref.start + 1;
        }

        if count % 2 == 0 {
            return false;
        }

        state.negated = true;
        state.start = state.start + 1;
        return true;
    };


}
