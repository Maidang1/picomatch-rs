mod test;
mod types;
mod utils;

use regex::{Regex, RegexBuilder};
use types::PicomatchOptions;
use utils::StringOrArray;

struct Picomatch {}

struct RegexOptions {
    nocase: Option<bool>,
    debug: Option<bool>,
}

impl Default for RegexOptions {
    fn default() -> Self {
        RegexOptions {
            nocase: None,
            debug: None,
        }
    }
}

impl Picomatch {
    /**
     * @name picomatch
     * @param {String|Array} `globs` One or more glob patterns.
     * @param {Object=} `options`
     */

    pub fn new(glob: StringOrArray, options: PicomatchOptions, return_state: Option<bool>) -> bool {
        let return_state = return_state.unwrap_or(false);

        if glob.is_array() {
            return false;
        }

        let posix = options.windows.unwrap_or(false);
        return false;
    }

    pub fn compileRe(
        &mut self,
        state: String,
        options: PicomatchOptions,
        return_state: Option<bool>,
    ) {
        // if return_state.unwrap_or(false) {
        //     return state.output;
        // }

        // let mut prepend = String::from("^");
        // let mut append = String::from("$");
        // let contains = options.contains.unwrap_or(false);
        // if contains {
        //     prepend = String::from("");
        //     append = String::from("");
        // }
        // let mut source = format!("{}(?:{}){}", prepend, state.output, append);

        // if state.negated {
        //     source = format!("^(?!{}).*$", source)
        // }
        // let regex = self.to_regex(source, options);
        // if return_state {
        //     return_state.state = state;
        // }

        // return regex;
    }
    pub fn to_regex(
        source: &str,
        options: Option<&RegexOptions>,
    ) -> Result<regex::Regex, regex::Error> {
        let binding = RegexOptions::default();
        let opts = options.unwrap_or(&binding);

        RegexBuilder::new(source)
            .case_insensitive(opts.nocase.unwrap_or(false))
            .build()
            .or_else(|err| {
                if opts.debug.unwrap_or(false) {
                    Err(err)
                } else {
                    Ok(Regex::new(r"$^").unwrap())
                }
            })
    }
}
