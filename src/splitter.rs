/*
 *    This file is part of Quick Reader.
 *
 *    Quick Reader is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    Quick Reader is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with Quick Reader.  If not, see <https://www.gnu.org/licenses/>.
 */

use unicode_segmentation::UnicodeSegmentation;

/// start_offset and stop_offset are [unicode grapheme offsets](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TextChunk {
    pub chunk: String,
    pub start_offset: usize,
    pub stop_offset: usize,
}

pub fn split(input: &str, chunk_size: usize) -> Vec<TextChunk> {
    let input = input.trim();
    if input.is_empty() {
        return vec![TextChunk {
            chunk: "".into(),
            start_offset: 0,
            stop_offset: 0,
        }];
    }

    let mut text_chunks = vec![];
    let mut prev_word_byte_end = 0;
    let mut prev_word_grapheme_length = 0;

    for word in input.split_whitespace() {
        let curr_word_byte_start = word.as_ptr() as usize - input.as_ptr() as usize;

        let whitespace_jumped = &input[prev_word_byte_end..curr_word_byte_start]
            .graphemes(true)
            .count();

        let curr_word_grapheme_length = word.graphemes(true).count();
        let curr_word_grapheme_start = whitespace_jumped + prev_word_grapheme_length;

        text_chunks.push(TextChunk {
            chunk: word.into(),
            start_offset: curr_word_grapheme_start,
            stop_offset: curr_word_grapheme_start + curr_word_grapheme_length,
        });

        prev_word_byte_end = curr_word_byte_start + word.len();
        prev_word_grapheme_length = curr_word_grapheme_start + curr_word_grapheme_length
    }

    if chunk_size > 1 {
        let mut new_text_chunks = Vec::with_capacity(text_chunks.len() / chunk_size);

        for section in text_chunks.chunks(chunk_size) {
            let joined_text = section
                .iter()
                .map(|s| s.chunk.clone())
                .reduce(|prev, curr| format!("{prev} {curr}"))
                .expect("to work");

            new_text_chunks.push(TextChunk {
                chunk: joined_text,
                start_offset: section.first().expect("to exist").start_offset,
                stop_offset: section.last().expect("to exist").stop_offset,
            });
        }

        return new_text_chunks;
    }

    text_chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let res = split("", 1);
        assert_eq!(
            res,
            vec![TextChunk {
                chunk: "".into(),
                start_offset: 0,
                stop_offset: 0,
            }]
        );
    }

    #[test]
    fn test_only_whitespace() {
        let res = split("  \n\n\n  　\n", 1);
        assert_eq!(
            res,
            vec![TextChunk {
                chunk: "".into(),
                start_offset: 0,
                stop_offset: 0,
            }]
        );
    }

    #[test]
    fn test_one_chunk() {
        let res = split(
            r#"Welcome to "Quick Reader". Press start to begin reading quickly."#,
            1,
        );

        assert_eq!(
            res,
            vec![
                TextChunk {
                    chunk: "Welcome".into(),
                    start_offset: 0,
                    stop_offset: 7
                },
                TextChunk {
                    chunk: "to".into(),
                    start_offset: 8,
                    stop_offset: 10
                },
                TextChunk {
                    chunk: "\"Quick".into(),
                    start_offset: 11,
                    stop_offset: 17
                },
                TextChunk {
                    chunk: "Reader\".".into(),
                    start_offset: 18,
                    stop_offset: 26
                },
                TextChunk {
                    chunk: "Press".into(),
                    start_offset: 27,
                    stop_offset: 32
                },
                TextChunk {
                    chunk: "start".into(),
                    start_offset: 33,
                    stop_offset: 38
                },
                TextChunk {
                    chunk: "to".into(),
                    start_offset: 39,
                    stop_offset: 41
                },
                TextChunk {
                    chunk: "begin".into(),
                    start_offset: 42,
                    stop_offset: 47
                },
                TextChunk {
                    chunk: "reading".into(),
                    start_offset: 48,
                    stop_offset: 55
                },
                TextChunk {
                    chunk: "quickly.".into(),
                    start_offset: 56,
                    stop_offset: 64
                }
            ]
        )
    }

    #[test]
    fn test_two_chunks() {
        let res = split(
            r#"Welcome to "Quick Reader". Press start to begin reading quickly."#,
            2,
        );

        assert_eq!(
            res,
            vec![
                TextChunk {
                    chunk: "Welcome to".into(),
                    start_offset: 0,
                    stop_offset: 10
                },
                TextChunk {
                    chunk: "\"Quick Reader\".".into(),
                    start_offset: 11,
                    stop_offset: 26
                },
                TextChunk {
                    chunk: "Press start".into(),
                    start_offset: 27,
                    stop_offset: 38
                },
                TextChunk {
                    chunk: "to begin".into(),
                    start_offset: 39,
                    stop_offset: 47
                },
                TextChunk {
                    chunk: "reading quickly.".into(),
                    start_offset: 48,
                    stop_offset: 64
                }
            ]
        )
    }

    #[test]
    fn test_mixed_unicode() {
        let res = split("こんにちは world и　皆さん", 1);

        assert_eq!(
            res,
            vec![
                TextChunk {
                    chunk: "こんにちは".into(),
                    start_offset: 0,
                    stop_offset: 5
                },
                TextChunk {
                    chunk: "world".into(),
                    start_offset: 6,
                    stop_offset: 11
                },
                TextChunk {
                    chunk: "и".into(),
                    start_offset: 12,
                    stop_offset: 13
                },
                TextChunk {
                    chunk: "皆さん".into(),
                    start_offset: 14,
                    stop_offset: 17
                },
            ]
        )
    }

    #[test]
    fn test_double_mixed_unicode() {
        let res = split("こんにちは world и　皆さん", 2);

        assert_eq!(
            res,
            vec![
                TextChunk {
                    chunk: "こんにちは world".into(),
                    start_offset: 0,
                    stop_offset: 11
                },
                TextChunk {
                    chunk: "и 皆さん".into(),
                    start_offset: 12,
                    stop_offset: 17
                },
            ]
        )
    }

    #[test]
    fn test_triple_mixed_unicode() {
        let res = split("こんにちは world и　皆さん", 3);

        assert_eq!(
            res,
            vec![
                TextChunk {
                    chunk: "こんにちは world и".into(),
                    start_offset: 0,
                    stop_offset: 13
                },
                TextChunk {
                    chunk: "皆さん".into(),
                    start_offset: 14,
                    stop_offset: 17
                },
            ]
        )
    }

    #[test]
    fn test_unicode_with_newlines() {
        let res = split("こんにちは world и　皆さん\n\n and ! everyone else", 1);

        assert_eq!(
            res,
            vec![
                TextChunk {
                    chunk: "こんにちは".into(),
                    start_offset: 0,
                    stop_offset: 5
                },
                TextChunk {
                    chunk: "world".into(),
                    start_offset: 6,
                    stop_offset: 11
                },
                TextChunk {
                    chunk: "и".into(),
                    start_offset: 12,
                    stop_offset: 13
                },
                TextChunk {
                    chunk: "皆さん".into(),
                    start_offset: 14,
                    stop_offset: 17
                },
                TextChunk {
                    chunk: "and".into(),
                    start_offset: 20,
                    stop_offset: 23
                },
                TextChunk {
                    chunk: "!".into(),
                    start_offset: 24,
                    stop_offset: 25
                },
                TextChunk {
                    chunk: "everyone".into(),
                    start_offset: 26,
                    stop_offset: 34
                },
                TextChunk {
                    chunk: "else".into(),
                    start_offset: 35,
                    stop_offset: 39
                },
            ]
        )
    }
}
