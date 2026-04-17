/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use crate::runner::RunnerError;
use hurl_core::ast::SourceInfo;
use hurl_core::error::DisplaySourceError;
use hurl_core::text::StyledString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputError(RunnerError);

impl From<RunnerError> for OutputError {
    fn from(error: RunnerError) -> Self {
        OutputError(error)
    }
}

/// Textual Output for runner errors
impl DisplaySourceError for OutputError {
    fn source_info(&self) -> SourceInfo {
        self.0.source_info
    }

    fn description(&self) -> String {
        self.0.description()
    }

    fn fixme(&self, content: &[&str]) -> StyledString {
        self.0.fixme(content)
    }
}
