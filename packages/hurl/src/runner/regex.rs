/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::collections::HashMap;

use hurl_core::ast::RegexValue;
use regex::Regex;

use crate::runner::template::eval_template;
use crate::runner::{Error, RunnerError, Value};

pub fn eval_regex_value(
    regex_value: &RegexValue,
    variables: &HashMap<String, Value>,
) -> Result<Regex, Error> {
    match regex_value {
        RegexValue::Template(t) => {
            let value = eval_template(t, variables)?;
            match Regex::new(value.as_str()) {
                Ok(re) => Ok(re),
                Err(_) => Err(Error {
                    source_info: t.source_info.clone(),
                    inner: RunnerError::InvalidRegex,
                    assert: false,
                }),
            }
        }
        RegexValue::Regex(re) => Ok(re.inner.clone()),
    }
}
