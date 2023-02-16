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
use super::Error;
use crate::runner::HurlResult;
use crate::util::path;
use hurl_core::parser;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase {
    id: String,
    pub filename: String,
    pub success: bool,
    pub time_in_ms: u128,
}

impl Testcase {
    /// Creates an HTML testcase.
    pub fn from(hurl_result: &HurlResult) -> Testcase {
        Testcase {
            id: "to-be-defined".to_string(),
            filename: hurl_result.filename.to_string(),
            time_in_ms: hurl_result.time_in_ms,
            success: hurl_result.success,
        }
    }

    /// Exports a [`Testcase`] to HTML.
    ///
    /// For the moment, it's just an export of this HTML file, with syntax colored.
    pub fn write_html(&self, content: &str, dir_path: &Path) -> Result<(), Error> {
        let relative_input_file = path::canonicalize_filename(&self.filename);
        let absolute_input_file = dir_path.join(format!("{relative_input_file}.html"));

        let parent = absolute_input_file.parent().expect("a parent");
        std::fs::create_dir_all(parent).unwrap();
        let mut file = match std::fs::File::create(&absolute_input_file) {
            Err(why) => {
                return Err(Error {
                    message: format!(
                        "Issue writing to {}: {:?}",
                        absolute_input_file.display(),
                        why
                    ),
                });
            }
            Ok(file) => file,
        };
        let hurl_file = parser::parse_hurl_file(content).expect("valid hurl file");

        let s = hurl_core::format::format_html(hurl_file, true);

        if let Err(why) = file.write_all(s.as_bytes()) {
            return Err(Error {
                message: format!(
                    "Issue writing to {}: {:?}",
                    absolute_input_file.display(),
                    why
                ),
            });
        }
        Ok(())
    }
}
