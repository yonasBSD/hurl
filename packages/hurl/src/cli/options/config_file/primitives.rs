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

use hurl_core::reader::Reader;

/// Skip whitespaces and comments
pub fn skip_whitespace_and_comments(reader: &mut Reader) {
    loop {
        reader.read_while(|c: char| c.is_whitespace());
        if reader.is_eof() {
            break;
        }
        if reader.peek() == Some('#') {
            // Skip comment line
            reader.read_while(|c: char| c != '\n');
            if reader.peek() == Some('\n') {
                reader.read(); // consume newline
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::reader::Pos;

    use super::*;

    #[test]
    fn test_skip_whitespace_and_comments() {
        let mut reader =
            Reader::new("# This is a comment\n   # Another comment\n   option=value\n");
        skip_whitespace_and_comments(&mut reader);
        assert_eq!((reader.cursor().pos), Pos::new(3, 4));
    }
}
