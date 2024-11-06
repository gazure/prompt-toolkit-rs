use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

use crate::input::ansi_escape_sequences::ANSI_SEQUENCES;
use crate::{input::base::KeyPress, keys::Keys};

static CURSOR_POSITION_RESPONSE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\x1b\[\d+;\d+R\z").expect("valid regex"));
static CURSOR_POSITION_RESPONSE_PREFIX_REGEX: LazyLock<Regex> =
    LazyLock::new(|| regex::Regex::new(r"^\x1b\[[\d;]*\z").expect("valid regex"));

static MOUSE_EVENT_PREFIX_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\x1b\[(<?[\d;]*|M.{0,2})\z").expect("valid regex"));
static MOUSE_EVENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\x1b\[(<?[\d;]+[mM]|M...)\z").expect("valid regex"));

#[derive(Debug, Copy, Clone)]
enum StateMachineInput {
    Character(char),
    Flush,
}

pub struct Parser {
    prefix: String,
    paste_buffer: String,
    prefix_match_cache: PrefixMatchCache,
    retry: bool,
    flush: bool,
    in_bracketed_paste: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            in_bracketed_paste: false,
            prefix: String::new(),
            paste_buffer: String::new(),
            prefix_match_cache: PrefixMatchCache::new(),
            retry: false,
            flush: false,
        }
    }

    pub fn feed(&mut self, data: &str) -> Vec<KeyPress> {
        let mut results = Vec::new();
        if self.in_bracketed_paste {
            self.paste_buffer.push_str(data);
            let end_mark = "\x1b[201~";
            if let Some(end_index) = self.paste_buffer.find(end_mark) {
                let paste_content = self.paste_buffer[..end_index].to_string();
                results.push(KeyPress::new(Keys::BracketedPaste, paste_content));

                self.in_bracketed_paste = false;
                let remaining = self.paste_buffer[end_index + end_mark.len()..].to_string();
                self.paste_buffer.clear();
                results.extend(self.feed(&remaining));
            }
        } else {
            for (i, c) in data.chars().enumerate() {
                if self.in_bracketed_paste {
                    results.extend(self.feed(&data[i..]));
                    break;
                }
                results.extend(self.send(StateMachineInput::Character(c)));
            }
        }

        results
    }

    /// Flush the buffer of the input stream.
    ///
    /// This will allow us to handle the escape key (or maybe meta) sooner.
    /// The input received by the escape key is actually the same as the first
    /// characters of e.g. Arrow-Up, so without knowing what follows the escape
    /// sequence, we don't know whether escape has been pressed, or whether
    /// it's something else. This flush function should be called after a
    /// timeout, and processes everything that's still in the buffer as-is, so
    /// without assuming any characters will follow.
    pub fn flush(&mut self) -> Vec<KeyPress> {
        self.send(StateMachineInput::Flush)
    }

    fn send(&mut self, c: StateMachineInput) -> Vec<KeyPress> {
        let mut result = Vec::new();
        match c {
            StateMachineInput::Flush => self.flush = true,
            StateMachineInput::Character(c) => {
                self.prefix.push(c);
            }
        }

        loop {
            if self.retry {
                self.flush = false;
                self.retry = false;
            }
            if !self.prefix.is_empty() {
                let is_prefix_match = self.prefix_match_cache.get(&self.prefix);
                let full_match = get_matches(&self.prefix);
                if (self.flush || !is_prefix_match) && full_match.is_some() {
                    self.handler(full_match, self.prefix.clone(), &mut result);
                    self.prefix = String::new();
                } else if (self.flush || !is_prefix_match) && full_match.is_none() {
                    let mut found = false;
                    self.retry = true;

                    for i in self.prefix.len()..0 {
                        if let Some(matches) = get_matches(&self.prefix[..i]) {
                            self.handler(Some(matches), self.prefix[..i].to_string(), &mut result);
                            self.prefix = self.prefix[i..].to_string();
                            found = true;
                        }
                    }

                    if !found {
                        let prefix = self.prefix.clone();
                        let mut chars = prefix.chars();
                        if let Some(c) = chars.next() {
                            self.handler(
                                Some(vec![Keys::Character(c)]),
                                c.to_string(),
                                &mut result,
                            );
                            self.prefix = chars.collect()
                        }
                    }
                }
            }
            if !self.retry {
                break;
            }
        }
        result
    }

    fn handler(
        &mut self,
        matches: Option<Vec<Keys>>,
        insert_text: String,
        results: &mut Vec<KeyPress>,
    ) {
        if let Some(keys) = matches {
            if keys.len() > 1 {
                keys.iter().enumerate().for_each(|(i, key)| {
                    self.handler(
                        Some(vec![*key]),
                        if i == 0 {
                            insert_text.clone()
                        } else {
                            String::default()
                        },
                        results,
                    );
                });
            } else {
                let key = keys.first().expect("vec of 1 key");
                if *key == Keys::BracketedPaste {
                    self.in_bracketed_paste = true;
                    self.paste_buffer = String::new();
                } else {
                    results.push(KeyPress::new(*key, insert_text));
                }
            }
        }
    }
}

fn get_matches(prefix: &str) -> Option<Vec<Keys>> {
    if CURSOR_POSITION_RESPONSE_REGEX.is_match(prefix) {
        Some(vec![Keys::CPRResponse])
    } else if MOUSE_EVENT_REGEX.is_match(prefix) {
        Some(vec![Keys::Vt100MouseEvent])
    } else {
        ANSI_SEQUENCES.get(prefix).cloned()
    }
}

#[derive(Debug)]
struct PrefixMatchCache {
    cache: HashMap<String, bool>,
}

impl PrefixMatchCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&mut self, prefix: &str) -> bool {
        if let Some(value) = self.cache.get(prefix) {
            *value
        } else {
            let result = CURSOR_POSITION_RESPONSE_PREFIX_REGEX.is_match(prefix)
                || MOUSE_EVENT_PREFIX_REGEX.is_match(prefix)
                || ANSI_SEQUENCES
                    .iter()
                    .any(|(k, _)| k.starts_with(prefix) && *k != prefix);
            self.cache.insert(prefix.to_string(), result);
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prefix_match_cache() {
        let mut cache = PrefixMatchCache::new();
        assert!(cache.get("\x1b["));
        assert!(cache.get("\x1b[1"));
        assert!(!cache.get("\x1b[2;1R]"));
        assert!(!cache.get("a"));
        assert!(cache.get("\x1b[<"));
        assert!(cache.get("\x1b[M"));
    }

    #[test]
    fn test_parser() {
        let mut parser = Parser::new();
        let results = parser.feed("hello");
        assert_eq!(results.len(), 5);
        assert_eq!(
            results[0],
            KeyPress::new(Keys::Character('h'), "h".to_string())
        );
        assert_eq!(
            results[1],
            KeyPress::new(Keys::Character('e'), "e".to_string())
        );
        assert_eq!(
            results[2],
            KeyPress::new(Keys::Character('l'), "l".to_string())
        );
        assert_eq!(
            results[3],
            KeyPress::new(Keys::Character('l'), "l".to_string())
        );
        assert_eq!(
            results[4],
            KeyPress::new(Keys::Character('o'), "o".to_string())
        );

        let results = parser.feed("\x1b[A");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], KeyPress::new(Keys::Up, "\x1b[A".to_string()));

        let results = parser.feed("\x1b[200~clipboard\x1b[201~");
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            KeyPress::new(Keys::BracketedPaste, "clipboard".to_string())
        );
    }
}
