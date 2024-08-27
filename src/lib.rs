mod test;
mod types;
mod utils;
mod parse;
mod constants;
mod scan;
use regex::{Regex, RegexBuilder};
use types::{CompileReResult, CompileReReturn, PicomatchOptions, State};
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
    fn to_regex(
        &self,
        source: &str,
        options: Option<&RegexOptions>,
    ) -> Result<Regex, regex::Error> {
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

    fn compile_re(
        &mut self,
        state: Option<State>,
        options: PicomatchOptions,
        return_output: Option<bool>,
        return_state: Option<bool>,
    ) -> CompileReReturn {
        let return_output = return_output.unwrap_or(false);
        let return_state = return_state.unwrap_or(false);
        let state = state.unwrap_or(State {
            output: String::from(""),
            negated: false,
        });
        if return_output {
            return CompileReReturn::String(state.output);
        }

        let mut prepend = String::from("^");
        let mut append = String::from("$");
        let contains = options.contains.unwrap_or(false);
        if contains {
            prepend = String::from("");
            append = String::from("");
        }
        let mut source = format!("{}(?:{}){}", prepend, state.output, append);
        if state.negated {
            source = format!("^(?!{}).*$", source);
        }
        let regex = self.to_regex(
            &source,
            Some(&RegexOptions {
                debug: options.debug,
                nocase: options.nocase,
            }),
        );
        let mut compile_re_result = CompileReResult { regex, state: None };
        if return_state {
            compile_re_result.state = Some(state);
        }

        return CompileReReturn::CompileReResult(compile_re_result);
    }

    fn mark_re(input: Option<String>) {
        if input.is_none() {
            panic!("Expected a non-empty string")
        }
        // let parsed = {negated: false, fastpaths: true};
    }

    // pub fn new(glob: StringOrArray, options: PicomatchOptions, return_state: Option<bool>) -> bool {
    //     let return_state = return_state.unwrap_or(false);

    //     if glob.is_array() {
    //         return false;
    //     }

    //     let posix = options.windows.unwrap_or(false);
    //     return false;
    // }

    // pub fn compileRe(
    //     &mut self,
    //     state: String,
    //     options: PicomatchOptions,
    //     return_state: Option<bool>,
    // ) {
    //     // if return_state.unwrap_or(false) {
    //     //     return state.output;
    //     // }

    //     // let mut prepend = String::from("^");
    //     // let mut append = String::from("$");
    //     // let contains = options.contains.unwrap_or(false);
    //     // if contains {
    //     //     prepend = String::from("");
    //     //     append = String::from("");
    //     // }
    //     // let mut source = format!("{}(?:{}){}", prepend, state.output, append);

    //     // if state.negated {
    //     //     source = format!("^(?!{}).*$", source)
    //     // }
    //     // let regex = self.to_regex(source, options);
    //     // if return_state {
    //     //     return_state.state = state;
    //     // }

    //     // return regex;
    // }
}
