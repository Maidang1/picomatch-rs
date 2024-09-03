use crate::parse::{fast_paths, ParseOptions};

#[derive(Default)]
struct Parsed {
    negated: bool,
    fast_paths: bool,
    output: Option<String>,
}

#[derive(Debug, Clone)]
enum PicomatchGlob {
    String(String),
    Array(Vec<String>),
}

impl PicomatchGlob {
    pub fn to_array(&self) -> Vec<String> {
        match self {
            PicomatchGlob::String(s) => vec![s.to_string()],
            PicomatchGlob::Array(arr) => arr.clone(),
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            PicomatchGlob::Array(_) => true,
            PicomatchGlob::String(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
struct PicomatchOptions {
    basename: Option<String>,
    bash: Option<bool>,
    capture: Option<bool>,
    contains: Option<bool>,
    posix: Option<bool>,
    cwd: Option<String>,
    debug: Option<bool>,
    fast_paths: Option<bool>,
    fail_glob: Option<bool>,
    flags: Option<String>,
    // expandRange?(from: string, to: string, options: PicomatchOptions): string;
    // expandRange?(from: string, to: string, step: string, options: PicomatchOptions): string;
    // format?: ((str: string) => string) | undefined;
    // onIgnore?: ((result: Result) => void) | undefined;
    // onMatch?: ((result: Result) => void) | undefined;
    // onResult?: ((result: Result) => void) | undefined;
    ignore: Option<PicomatchGlob>,
    keep_quotes: Option<bool>,
    literal_brackets: Option<bool>,
    lookbehinds: Option<bool>,
    match_base: Option<bool>,
    max_length: Option<bool>,
    nobrace: Option<bool>,
    nobracket: Option<bool>,
    nocase: Option<bool>,
    /**
     * @deprecated use `nounique` instead.
     */
    nodupes: Option<bool>,
    noext: Option<bool>,
    noextglob: Option<bool>,
    noglobstar: Option<bool>,
    nonegate: Option<bool>,
    noquantifiers: Option<bool>,
    posix_slashes: Option<bool>,
    prepend: Option<String>,
    regex: Option<bool>,
    strict_brackets: Option<bool>,
    strict_slashes: Option<bool>,
    unescape: Option<bool>,
    windows: Option<bool>,
}

impl PicomatchOptions {
    pub fn to_parse_options(&self) -> ParseOptions {
        ParseOptions {
            max_length: None,
            prepend: self.prepend.clone(),
            windows: self.windows.unwrap_or(false),
            capture: Some(self.capture.unwrap_or(false)),
            dot: Some(false),
            bash: Some(self.bash.unwrap_or(false)),
            noext: Some(self.noext.unwrap_or(false)),
            noextglob: Some(self.noextglob.unwrap_or(false)),
            noglobstar: Some(self.noglobstar.unwrap_or(false)),
            strict_slashes: Some(self.strict_slashes.unwrap_or(false)),
        }
    }
}

pub fn picomatch(glob: &PicomatchGlob, options: &PicomatchOptions) -> Option<bool> {
    if glob.is_array() {
        println!("todo impl Array: {:?}", glob.to_array());
        return None;
    }
    // const isState = isObject(glob) && glob.tokens && glob.input;

    let posix = options.posix.unwrap_or(false);
    /**
     *   const regex = isState
    ?     picomatch.compileRe(glob, options)
    :     picomatch.makeRe(glob, options, false, true);
     */
    None
}

fn make_re(
    input: &String,
    options: &PicomatchOptions,
    return_output: Option<bool>,
    return_state: Option<bool>,
)->Option<bool> {
    if input.is_empty() {
        panic!("Expected a non-empty string");
    }

    let mut parsed = Parsed::default();

    if options.fast_paths == Some(true)
        && (input.chars().nth(0) == Some('.') || input.chars().nth(1) == Some('*'))
    {
        parsed.output = fast_paths(input, &options.to_parse_options());
    }
    if parsed.output.is_none() {
        // parsed = parse(input, options);
    }

    return compile_re(&parsed, &options, return_output, return_state);
}

fn compile_re(
    parsed: &Parsed,
    options: &PicomatchOptions,
    return_output: Option<bool>,
    return_state: Option<bool>,
) -> Option<bool> {
    return None;
}

// fn to_regex(source: &str, options: Option<&RegexOptions>) -> Result<Regex, regex::Error> {
//     let default_opts = RegexOptions::default();
//     let opts = options.unwrap_or(&default_opts);

//     RegexBuilder::new(source)
//         .case_insensitive(opts.nocase.unwrap_or(false))
//         .build()
//         .or_else(|err| {
//             if opts.debug.unwrap_or(false) {
//                 Err(err)
//             } else {
//                 Ok(Regex::new(r"$^").unwrap())
//             }
//         })
// }
