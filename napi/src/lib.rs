use napi::{
    Env, Error, JsFunction, JsObject, JsUnknown, Result, Status, ValueType,
};
use napi_derive::napi;
use picomatch_rs::{
    make_re as make_re_impl, parse as parse_impl, CompileOptions, ParseState, ParseToken,
    RegexDescriptor, ScanOptions, ScanState, ScanToken,
};
use serde_json::Value;

fn normalize_compile_options_value(mut value: Option<Value>) -> Option<Value> {
    if let Some(Value::Object(object)) = value.as_mut() {
        if !object.contains_key("noextglob") {
            if let Some(Value::Bool(enabled)) = object.get("noext") {
                object.insert("noextglob".to_string(), Value::Bool(*enabled));
            }
        }
    }

    value
}

fn compile_options_from_value(value: Option<Value>) -> Result<CompileOptions> {
    normalize_compile_options_value(value)
        .map(serde_json::from_value::<CompileOptions>)
        .transpose()?
        .map(Ok)
        .unwrap_or_else(|| Ok(CompileOptions::default()))
}

fn scan_options_from_value(value: Option<Value>) -> Result<ScanOptions> {
    value
        .map(serde_json::from_value::<ScanOptions>)
        .transpose()?
        .map(Ok)
        .unwrap_or_else(|| Ok(ScanOptions::default()))
}

fn bool_option(value: &Option<Value>, key: &str) -> bool {
    match value {
        Some(Value::Object(object)) => object.get(key).and_then(Value::as_bool).unwrap_or(false),
        _ => false,
    }
}

fn flags_for_options(options: &CompileOptions) -> String {
    if !options.flags.is_empty() {
        options.flags.clone()
    } else if options.nocase {
        "i".to_string()
    } else {
        String::new()
    }
}

fn descriptor_from_state(
    state: ParseState,
    options: &CompileOptions,
    return_state: bool,
) -> RegexDescriptor {
    let prepend = if options.contains { "" } else { "^" };
    let append = if options.contains { "" } else { "$" };
    let output = state.output.clone();
    let mut source = format!("{prepend}(?:{output}){append}");

    if state.negated {
        source = format!("^(?!{source}).*$");
    }

    RegexDescriptor {
        source,
        flags: flags_for_options(options),
        output,
        state: return_state.then_some(state),
    }
}

fn parse_patterns(value: Value) -> Result<Vec<String>> {
    match value {
        Value::String(pattern) => Ok(vec![pattern]),
        Value::Array(values) => values
            .into_iter()
            .map(|value| match value {
                Value::String(pattern) => Ok(pattern),
                _ => Err(Error::new(
                    Status::InvalidArg,
                    "Expected pattern to be a string or an array of strings".to_string(),
                )),
            })
            .collect(),
        _ => Err(Error::new(
            Status::InvalidArg,
            "Expected pattern to be a string or an array of strings".to_string(),
        )),
    }
}

fn ensure_non_empty_pattern(pattern: &str) -> Result<()> {
    if pattern.is_empty() {
        return Err(Error::new(
            Status::InvalidArg,
            "Expected pattern to be a non-empty string".to_string(),
        ));
    }

    Ok(())
}

fn regex_input_from_value(value: JsUnknown) -> Result<(String, String)> {
    match value.get_type()? {
        ValueType::String => {
            let string = unsafe { value.cast::<napi::JsString>() };
            Ok((string.into_utf8()?.as_str()?.to_string(), String::new()))
        }
        ValueType::Object | ValueType::Function => {
            let object = value.coerce_to_object()?;
            let source = object.get_named_property::<String>("source")?;
            let flags = object
                .get_named_property::<String>("flags")
                .unwrap_or_default();
            Ok((source, flags))
        }
        _ => Err(Error::new(
            Status::InvalidArg,
            "Expected a RegExp, descriptor object, or regex source string".to_string(),
        )),
    }
}

fn to_posix_slashes(input: &str) -> String {
    input.replace('\\', "/")
}

fn basename(input: &str, windows: bool) -> String {
    let parts: Vec<&str> = if windows {
        input.split(['/', '\\']).collect()
    } else {
        input.split('/').collect()
    };

    match parts.last().copied() {
        Some("") => parts
            .get(parts.len().saturating_sub(2))
            .copied()
            .unwrap_or_default()
            .to_string(),
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

fn create_regex_object(env: &Env, source: &str, flags: &str) -> Result<JsObject> {
    let global = env.get_global()?;
    let constructor = global.get_named_property::<JsFunction>("RegExp")?;
    let source_arg = env.create_string(source)?;

    if flags.is_empty() {
        constructor.new_instance(&[source_arg])
    } else {
        let flags_arg = env.create_string(flags)?;
        constructor.new_instance(&[source_arg, flags_arg])
    }
}

fn create_never_match_regex(env: &Env) -> Result<JsObject> {
    create_regex_object(env, "$^", "")
}

fn exec_regex(env: &Env, regex: &JsObject, input: &str) -> Result<Option<JsObject>> {
    let exec = regex.get_named_property::<JsFunction>("exec")?;
    let input_arg = env.create_string(input)?;
    let value = exec.call(Some(regex), &[input_arg])?;

    match value.get_type()? {
        ValueType::Null | ValueType::Undefined => Ok(None),
        _ => Ok(Some(value.coerce_to_object()?)),
    }
}

fn match_value_unknown(env: &Env, value: Option<JsObject>, exact: bool) -> Result<JsUnknown> {
    if let Some(value) = value {
        return Ok(value.into_unknown());
    }

    if exact {
        return Ok(env.get_boolean(true)?.into_unknown());
    }

    Ok(env.get_null()?.into_unknown())
}

fn create_result_object(
    env: &Env,
    glob: &str,
    descriptor: &RegexDescriptor,
    regex: &JsObject,
    input: &str,
    output: &str,
    posix: bool,
    exact: bool,
    match_value: Option<JsObject>,
    is_match: bool,
) -> Result<JsObject> {
    let mut result = env.create_object()?;
    result.set_named_property("glob", glob.to_string())?;

    if let Some(state) = &descriptor.state {
        result.set_named_property("state", env.to_js_value(state)?)?;
    }

    result.set_named_property("regex", regex)?;
    result.set_named_property("posix", posix)?;
    result.set_named_property("input", input.to_string())?;
    result.set_named_property("output", output.to_string())?;
    result.set_named_property("match", match_value_unknown(env, match_value, exact)?)?;
    result.set_named_property("isMatch", is_match)?;
    Ok(result)
}

fn execute_pattern(
    env: &Env,
    input: &str,
    glob: &str,
    descriptor: &RegexDescriptor,
    options: &CompileOptions,
    capture: bool,
    posix: bool,
) -> Result<JsObject> {
    let regex = create_regex_object(env, &descriptor.source, &descriptor.flags)?;

    if input.is_empty() {
        return create_result_object(
            env,
            glob,
            descriptor,
            &regex,
            input,
            "",
            posix,
            false,
            None,
            false,
        );
    }

    let mut exact = input == glob;
    let mut output = if exact && posix {
        to_posix_slashes(input)
    } else {
        input.to_string()
    };

    if !exact {
        output = if posix {
            to_posix_slashes(input)
        } else {
            input.to_string()
        };
        exact = output == glob;
    }

    let mut match_value = None;
    let mut is_match = exact;

    if !exact || capture {
        let candidate = if options.match_base || options.basename {
            basename(input, options.windows)
        } else {
            output.clone()
        };

        match_value = exec_regex(env, &regex, &candidate)?;
        is_match = match_value.is_some();
        exact = false;
    }

    create_result_object(
        env,
        glob,
        descriptor,
        &regex,
        input,
        &output,
        posix,
        exact,
        match_value,
        is_match,
    )
}

fn parse_token_to_js(env: &Env, token: &ParseToken) -> Result<JsObject> {
    let mut object = env.create_object()?;
    object.set_named_property("type", token.kind.clone())?;
    object.set_named_property("value", token.value.clone())?;

    if let Some(output) = &token.output {
        object.set_named_property("output", output.clone())?;
    }

    Ok(object)
}

fn parse_state_to_js(env: &Env, state: &ParseState) -> Result<JsObject> {
    let mut object = env.create_object()?;
    object.set_named_property("input", state.input.clone())?;
    object.set_named_property("output", state.output.clone())?;
    object.set_named_property("negated", state.negated)?;
    object.set_named_property("fastpaths", state.fastpaths)?;

    if let Some(tokens) = &state.tokens {
        let mut js_tokens = env.create_array_with_length(tokens.len())?;
        for (index, token) in tokens.iter().enumerate() {
            js_tokens.set_element(index as u32, parse_token_to_js(env, token)?)?;
        }
        object.set_named_property("tokens", js_tokens)?;
    }

    Ok(object)
}

fn regex_descriptor_to_js(env: &Env, descriptor: &RegexDescriptor) -> Result<JsObject> {
    let mut object = env.create_object()?;
    object.set_named_property("source", descriptor.source.clone())?;
    object.set_named_property("flags", descriptor.flags.clone())?;
    object.set_named_property("output", descriptor.output.clone())?;

    if let Some(state) = &descriptor.state {
        object.set_named_property("state", parse_state_to_js(env, state)?)?;
    }

    Ok(object)
}

fn scan_token_to_js(env: &Env, token: &ScanToken) -> Result<JsObject> {
    let mut object = env.create_object()?;
    object.set_named_property("value", token.value.clone())?;
    object.set_named_property("depth", token.depth)?;
    object.set_named_property("isGlob", token.is_glob)?;

    if let Some(value) = token.is_globstar {
        object.set_named_property("isGlobstar", value)?;
    }
    if let Some(value) = token.is_brace {
        object.set_named_property("isBrace", value)?;
    }
    if let Some(value) = token.is_bracket {
        object.set_named_property("isBracket", value)?;
    }
    if let Some(value) = token.is_extglob {
        object.set_named_property("isExtglob", value)?;
    }
    if let Some(value) = token.negated {
        object.set_named_property("negated", value)?;
    }
    if let Some(value) = token.backslashes {
        object.set_named_property("backslashes", value)?;
    }
    if let Some(value) = token.is_prefix {
        object.set_named_property("isPrefix", value)?;
    }

    Ok(object)
}

fn scan_state_to_js(env: &Env, state: ScanState) -> Result<JsObject> {
    let mut object = env.create_object()?;
    object.set_named_property("prefix", state.prefix)?;
    object.set_named_property("input", state.input)?;
    object.set_named_property("start", state.start as u32)?;
    object.set_named_property("base", state.base)?;
    object.set_named_property("glob", state.glob)?;
    object.set_named_property("isBrace", state.is_brace)?;
    object.set_named_property("isBracket", state.is_bracket)?;
    object.set_named_property("isGlob", state.is_glob)?;
    object.set_named_property("isExtglob", state.is_extglob)?;
    object.set_named_property("isGlobstar", state.is_globstar)?;
    object.set_named_property("negated", state.negated)?;
    object.set_named_property("negatedExtglob", state.negated_extglob)?;

    if let Some(max_depth) = state.max_depth {
        object.set_named_property("maxDepth", max_depth)?;
    }

    if let Some(tokens) = state.tokens {
        let mut js_tokens = env.create_array_with_length(tokens.len())?;
        for (index, token) in tokens.iter().enumerate() {
            js_tokens.set_element(index as u32, scan_token_to_js(env, token)?)?;
        }
        object.set_named_property("tokens", js_tokens)?;
    }

    if let Some(slashes) = state.slashes {
        let values = slashes
            .into_iter()
            .map(|value| value as u32)
            .collect::<Vec<_>>();
        object.set_named_property("slashes", values)?;
    }

    if let Some(parts) = state.parts {
        object.set_named_property("parts", parts)?;
    }

    Ok(object)
}

#[napi]
pub struct NativeMatcher {
    patterns: Vec<String>,
    descriptors: Vec<RegexDescriptor>,
    options: CompileOptions,
    capture: bool,
    posix: bool,
}

#[napi]
impl NativeMatcher {
    #[napi]
    pub fn test(&self, env: Env, input: String, return_object: Option<bool>) -> Result<JsUnknown> {
        let should_return_object = return_object.unwrap_or(false);

        for (pattern, descriptor) in self.patterns.iter().zip(self.descriptors.iter()) {
            let result = execute_pattern(
                &env,
                &input,
                pattern,
                descriptor,
                &self.options,
                self.capture,
                self.posix,
            )?;
            let is_match = result.get_named_property::<bool>("isMatch")?;

            if should_return_object {
                if is_match {
                    return Ok(result.into_unknown());
                }
            } else if is_match {
                return Ok(env.get_boolean(true)?.into_unknown());
            }
        }

        if should_return_object {
            if let Some((pattern, descriptor)) = self.patterns.iter().zip(self.descriptors.iter()).next() {
                return Ok(
                    execute_pattern(
                        &env,
                        &input,
                        pattern,
                        descriptor,
                        &self.options,
                        self.capture,
                        self.posix,
                    )?
                    .into_unknown(),
                );
            }
        }

        Ok(env.get_boolean(false)?.into_unknown())
    }

    #[napi(getter)]
    pub fn state(&self, env: Env) -> Result<JsUnknown> {
        if self.descriptors.len() == 1 {
            if let Some(state) = &self.descriptors[0].state {
                return env.to_js_value(state);
            }

            return Ok(env.get_null()?.into_unknown());
        }

        let states = self
            .descriptors
            .iter()
            .map(|descriptor| descriptor.state.clone())
            .collect::<Vec<_>>();
        env.to_js_value(&states)
    }

    #[napi(getter)]
    pub fn regex(&self, env: Env) -> Result<JsUnknown> {
        if self.descriptors.len() == 1 {
            let regex = create_regex_object(
                &env,
                &self.descriptors[0].source,
                &self.descriptors[0].flags,
            )?;
            return Ok(regex.into_unknown());
        }

        let mut values = env.create_array_with_length(self.descriptors.len())?;
        for (index, descriptor) in self.descriptors.iter().enumerate() {
            values.set_element(
                index as u32,
                create_regex_object(&env, &descriptor.source, &descriptor.flags)?,
            )?;
        }
        Ok(values.into_unknown())
    }
}

#[napi]
pub fn scan(env: Env, input: String, options: Option<Value>) -> Result<JsObject> {
    let options = scan_options_from_value(options)?;
    scan_state_to_js(&env, picomatch_rs::scan(&input, &options))
}

#[napi]
pub fn parse(env: Env, input: Value, options: Option<Value>) -> Result<JsUnknown> {
    let options = compile_options_from_value(options)?;

    match input {
        Value::String(pattern) => match parse_impl(&pattern, &options) {
            Some(state) => Ok(parse_state_to_js(&env, &state)?.into_unknown()),
            None => Ok(env.get_null()?.into_unknown()),
        },
        Value::Array(patterns) => {
            let mut values = env.create_array_with_length(patterns.len())?;

            for (index, pattern) in patterns.into_iter().enumerate() {
                match pattern {
                    Value::String(pattern) => {
                        let value = match parse_impl(&pattern, &options) {
                            Some(state) => parse_state_to_js(&env, &state)?.into_unknown(),
                            None => env.get_null()?.into_unknown(),
                        };
                        values.set_element(index as u32, value)?;
                    }
                    _ => {
                        return Err(Error::new(
                            Status::InvalidArg,
                            "Expected pattern to be a string or an array of strings".to_string(),
                        ));
                    }
                }
            }

            Ok(values.into_unknown())
        }
        _ => Err(Error::new(
            Status::InvalidArg,
            "Expected pattern to be a string or an array of strings".to_string(),
        )),
    }
}

#[napi(js_name = "compileRe")]
pub fn compile_re(
    env: Env,
    state: Value,
    options: Option<Value>,
    return_output: Option<bool>,
    return_state: Option<bool>,
) -> Result<JsUnknown> {
    let options = compile_options_from_value(options)?;
    let state = serde_json::from_value::<ParseState>(state)?;

    if return_output.unwrap_or(false) {
        return Ok(env.create_string(&state.output)?.into_unknown());
    }

    let descriptor = descriptor_from_state(state, &options, return_state.unwrap_or(false));
    Ok(regex_descriptor_to_js(&env, &descriptor)?.into_unknown())
}

#[napi(js_name = "makeRe")]
pub fn make_re(
    env: Env,
    input: String,
    options: Option<Value>,
    return_output: Option<bool>,
    return_state: Option<bool>,
) -> Result<JsUnknown> {
    let options = compile_options_from_value(options)?;
    ensure_non_empty_pattern(&input)?;

    let Some(descriptor) = make_re_impl(&input, &options, return_state.unwrap_or(false)) else {
        return Ok(env.get_null()?.into_unknown());
    };

    if return_output.unwrap_or(false) {
        return Ok(env.create_string(&descriptor.output)?.into_unknown());
    }

    Ok(regex_descriptor_to_js(&env, &descriptor)?.into_unknown())
}

#[napi(js_name = "toRegex")]
pub fn to_regex(env: Env, source: String, options: Option<Value>) -> Result<JsObject> {
    let debug = bool_option(&options, "debug");
    let options = compile_options_from_value(options)?;
    let flags = flags_for_options(&options);

    match create_regex_object(&env, &source, &flags) {
        Ok(regex) => Ok(regex),
        Err(err) if debug => Err(err),
        Err(_) => create_never_match_regex(&env),
    }
}

#[napi(js_name = "test")]
pub fn test_regex(
    env: Env,
    input: String,
    regex: JsUnknown,
    _options: Option<Value>,
) -> Result<JsObject> {
    let (source, flags) = regex_input_from_value(regex)?;
    let regex = create_regex_object(&env, &source, &flags)?;

    if input.is_empty() {
        return create_result_object(
            &env,
            "",
            &RegexDescriptor {
                source,
                flags,
                output: String::new(),
                state: None,
            },
            &regex,
            &input,
            "",
            false,
            false,
            None,
            false,
        );
    }

    let output = input.clone();
    let match_value = exec_regex(&env, &regex, &output)?;
    let is_match = match_value.is_some();

    create_result_object(
        &env,
        "",
        &RegexDescriptor {
            source,
            flags,
            output: String::new(),
            state: None,
        },
        &regex,
        &input,
        &output,
        false,
        false,
        match_value,
        is_match,
    )
}

#[napi(js_name = "matchBase")]
pub fn match_base(
    env: Env,
    input: String,
    glob: JsUnknown,
    options: Option<Value>,
) -> Result<bool> {
    let options = compile_options_from_value(options)?;
    let regex = match glob.get_type()? {
        ValueType::String => {
            let string = unsafe { glob.cast::<napi::JsString>() };
            let pattern = string.into_utf8()?.as_str()?.to_string();
            ensure_non_empty_pattern(&pattern)?;
            let descriptor = make_re_impl(&pattern, &options, false).ok_or_else(|| {
                Error::new(
                    Status::GenericFailure,
                    format!("Native makeRe does not support pattern: {pattern}"),
                )
            })?;
            create_regex_object(&env, &descriptor.source, &descriptor.flags)?
        }
        _ => {
            let (source, flags) = regex_input_from_value(glob)?;
            create_regex_object(&env, &source, &flags)?
        }
    };

    Ok(exec_regex(&env, &regex, &basename(&input, options.windows))?.is_some())
}

#[napi(js_name = "isMatch")]
pub fn is_match(env: Env, input: String, patterns: Value, options: Option<Value>) -> Result<bool> {
    let patterns = parse_patterns(patterns)?;
    let capture = bool_option(&options, "capture");
    let options = compile_options_from_value(options.clone())?;
    let posix = options.windows;

    for pattern in patterns {
        ensure_non_empty_pattern(&pattern)?;
        let descriptor = make_re_impl(&pattern, &options, true).ok_or_else(|| {
            Error::new(
                Status::GenericFailure,
                format!("Native makeRe does not support pattern: {pattern}"),
            )
        })?;
        let result =
            execute_pattern(&env, &input, &pattern, &descriptor, &options, capture, posix)?;
        if result.get_named_property::<bool>("isMatch")? {
            return Ok(true);
        }
    }

    Ok(false)
}

#[napi(js_name = "compileMatcher")]
pub fn compile_matcher(patterns: Value, options: Option<Value>) -> Result<NativeMatcher> {
    let capture = bool_option(&options, "capture");
    let options = compile_options_from_value(options)?;
    let patterns = parse_patterns(patterns)?;
    let mut descriptors = Vec::with_capacity(patterns.len());

    for pattern in &patterns {
        ensure_non_empty_pattern(pattern)?;
        let descriptor = make_re_impl(pattern, &options, true).ok_or_else(|| {
            Error::new(
                Status::GenericFailure,
                format!("Native makeRe does not support pattern: {pattern}"),
            )
        })?;
        descriptors.push(descriptor);
    }

    Ok(NativeMatcher {
        patterns,
        descriptors,
        posix: options.windows,
        capture,
        options,
    })
}
