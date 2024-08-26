use crate::constants::*;

struct Options {
    max_length: Option<i32>,
    prepend: Option<String>,
    capture: Option<bool>,
}

struct Bos {
    value: String,
    output: String,
    bos_type: String,
}

struct Parse {}

impl Parse {
    fn parse(&self, input: String, options: Options) {
        let mut input = input.as_str();
        if let Some(replacement) = REPLACEMENTS.get(input) {
            input = replacement;
        }
        let mut max = MAX_LENGTH.clone();
        if let Some(option_max) = options.max_length {
            if option_max < max {
                max = option_max;
            }
        }
        let len = input.len();

        let bos = Bos {
            bos_type: "bos".to_string(),
            value: "".to_string(),
            output: options.prepend.unwrap_or("".to_string()),
        };

        let mut tokens: Vec<Bos> = vec![bos];

        let capture = if options.capture.unwrap_or(false) {
            ""
        } else {
            "?:"
        };
    }
}
