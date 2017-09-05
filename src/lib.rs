//! The `grok` library allows you to quickly parse and match potentially unstructured data
//! into a structed result. It is especially helpful when parsing logfiles of all kinds. This
//! [Rust](http://rust-lang.org) version is mainly a port from the [java version](https://github.com/thekrakken/java-grok)
//! which in drew inspiration from the original [ruby version](https://github.com/logstash-plugins/logstash-filter-grok).
#![doc(html_root_url = "https://docs.rs/grok/0.1.0")]
extern crate regex;

use regex::{Captures, Regex};
use std::collections::BTreeMap;

const GROK_PATTERN: &'static str = r"%\{(?P<name>(?P<pattern>[A-z0-9]+)(?::(?P<subname>[A-z0-9_:;/\s\.]+))?)(?:=(?P<definition>(?:(?:[^{}]+|\.+)+)+))?\}";

/// The `Matches` represent matched results from a `Pattern` against text.
pub struct Matches<'a> {
    captures: Captures<'a>,
    alias: &'a BTreeMap<String, String>
}

impl<'a> Matches<'a> {
    /// Instantiates the matches for a pattern after the match.
    pub fn new(captures: Captures<'a>, alias: &'a BTreeMap<String, String>) -> Self {
        Matches { captures: captures, alias: alias }
    }

    /// Gets the value for the name (or) alias if found, `None` otherwise.
    pub fn get(&self, name: &str) -> Option<&str> {
        match self.alias.get(name) {
            Some(real) => self.captures.name(real).map(|m| m.as_str()),
            None => None,
        }
    }

    /// Returns the number of matches.
    pub fn len(&self) -> usize {
        self.captures.len() - 1
    }

    /// Returns true if there are no matches, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
} 

/// The `Pattern` represents a compiled regex, ready to be matched against arbitrary text.
pub struct Pattern {
    regex: Regex,
    alias: BTreeMap<String, String>,
}

impl Pattern {
    /// Creates a new pattern from a raw regex string and an alias map to identify the
    /// fields properly.
    pub fn new(regex: &str, alias: BTreeMap<String, String>) -> Self {
        Pattern {
            regex: Regex::new(regex).expect("Could not compile regex!"),
            alias: alias,
        }
    }

    /// Matches this compiled `Pattern` against the text and returns the matches.
    pub fn match_against<'a>(&'a self, text: &'a str) -> Option<Matches<'a>> {
        self.regex.captures(text).map(|cap| Matches::new(cap, &self.alias))
    }
}

/// The basic structure to manage patterns, entry point for common usage.
pub struct Grok {
    definitions: BTreeMap<String, String>,
}

impl Grok {
    /// Creates a new `Grok` instance with no patterns.
    pub fn empty() -> Self {
        Grok {
            definitions: BTreeMap::new(),
        }
    }

    /// Inserts a custom pattern.
    pub fn insert_definition<S: Into<String>>(&mut self, name: S, pattern: S) {
        self.definitions.insert(name.into(), pattern.into());
    }

    /// Compiles the given pattern, making it ready for matching.
    pub fn compile(&mut self, pattern: &str, named_only: bool) -> Pattern {
        let mut named_regex = String::from(pattern);
        let original_grok_pattern = pattern;
        let mut alias: BTreeMap<String, String> = BTreeMap::new();

        let mut index = 0;
        let mut iteration_left = 1000;
        let mut continue_iteration = true;

        let grok_regex = Regex::new(GROK_PATTERN).unwrap();
        while continue_iteration {
            continue_iteration = false;
            if iteration_left <= 0 {
                panic!(
                    "Deep recursion pattern compilation of {:?}",
                    original_grok_pattern
                );
            }
            iteration_left -= 1;

            let capture = named_regex.clone();
            if let Some(m) = grok_regex.captures(&capture) {
                continue_iteration = true;
            
                let mut name = String::from(m.name("name").unwrap().as_str());
                if let Some(definition) = m.name("definition") {
                    self.insert_definition(m.name("pattern").unwrap().as_str(), definition.as_str());
                    name = format!("{}={}", name, definition.as_str());
                }

                let n = format!("%{{{}}}", name);
                let count = named_regex.matches(&n).count();
                for _ in 0..count {
                    let definition_of_pattern = self.definitions.get(m.name("pattern").unwrap().as_str());
                    if definition_of_pattern.is_none() {
                        panic!("No definition for key '{}' found, aborting", m.name("pattern").unwrap().as_str());
                    }

                    let replacement = if named_only && m.name("subname").is_none() {
                        format!("(?:{})", definition_of_pattern.unwrap())
                    } else {
                        format!("(?P<name{}>{})", index, definition_of_pattern.unwrap())
                    };

                    let name_and_index = format!("name{}", index);
                    if m.name("subname").is_some() {
                        alias.insert(m.name("subname").unwrap().as_str().into(), name_and_index);
                    } else {
                        alias.insert(name.clone(), name_and_index);
                    }

                    let search_string = format!("%{{{}}}", name);
                    named_regex = named_regex.replacen(&search_string, &replacement, 1);
                    index += 1;
                }
            }
        }

        if named_regex.is_empty() {
            panic!("Pattern not found");
        }
        Pattern::new(&named_regex, alias)
    }
}

impl Default for Grok {
    fn default() -> Grok {
        Grok::empty()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_anonymous_pattern() {
        let mut grok = Grok::default();
        grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
        let pattern = grok.compile("%{USERNAME}", false);

        let matches = pattern.match_against("root").expect("No matches found!");
        assert_eq!("root", matches.get("USERNAME").unwrap());
        assert_eq!(1, matches.len());
        let matches = pattern.match_against("john doe").expect("No matches found!");
        assert_eq!("john", matches.get("USERNAME").unwrap());
        assert_eq!(1, matches.len());
    }

    #[test]
    fn test_simple_named_pattern() {
        let mut grok = Grok::default();
        grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
        let pattern = grok.compile("%{USERNAME:usr}", false);

        let matches = pattern.match_against("root").expect("No matches found!");
        assert_eq!("root", matches.get("usr").unwrap());
        assert_eq!(1, matches.len());
        let matches = pattern.match_against("john doe").expect("No matches found!");
        assert_eq!("john", matches.get("usr").unwrap());
        assert_eq!(1, matches.len());
    }

    #[test]
    fn test_alias_anonymous_pattern() {
        let mut grok = Grok::default();
        grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
        grok.insert_definition("USER", r"%{USERNAME}");
        let pattern = grok.compile("%{USER}", false);

        let matches = pattern.match_against("root").expect("No matches found!");
        assert_eq!("root", matches.get("USER").unwrap());
        let matches = pattern.match_against("john doe").expect("No matches found!");
        assert_eq!("john", matches.get("USER").unwrap());
    }

    #[test]
    fn test_ailas_named_pattern() {
        let mut grok = Grok::default();
        grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
        grok.insert_definition("USER", r"%{USERNAME}");
        let pattern = grok.compile("%{USER:usr}", false);

        let matches = pattern.match_against("root").expect("No matches found!");
        assert_eq!("root", matches.get("usr").unwrap());
        let matches = pattern.match_against("john doe").expect("No matches found!");
        assert_eq!("john", matches.get("usr").unwrap());
    }

    #[test]
    fn test_composite_or_pattern() {
        let mut grok = Grok::default();
        grok.insert_definition("MAC", r"(?:%{CISCOMAC}|%{WINDOWSMAC}|%{COMMONMAC})");
        grok.insert_definition("CISCOMAC", r"(?:(?:[A-Fa-f0-9]{4}\.){2}[A-Fa-f0-9]{4})");
        grok.insert_definition("WINDOWSMAC", r"(?:(?:[A-Fa-f0-9]{2}-){5}[A-Fa-f0-9]{2})");
        grok.insert_definition("COMMONMAC", r"(?:(?:[A-Fa-f0-9]{2}:){5}[A-Fa-f0-9]{2})");
        let pattern = grok.compile("%{MAC}", false);

        let matches = pattern.match_against("5E:FF:56:A2:AF:15").expect("No matches found!");
        assert_eq!("5E:FF:56:A2:AF:15", matches.get("MAC").unwrap());
        assert_eq!(4, matches.len());
        let matches = pattern.match_against("hello! 5E:FF:56:A2:AF:15 what?").expect("No matches found!");
        assert_eq!("5E:FF:56:A2:AF:15", matches.get("MAC").unwrap());
        assert_eq!(true, pattern.match_against("5E:FF").is_none());
    }

    #[test]
    fn test_multiple_patterns() {
        let mut grok = Grok::default();
        grok.insert_definition("YEAR", r"(\d\d){1,2}");
        grok.insert_definition("MONTH", r"\b(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)\b");
        grok.insert_definition("DAY", r"(?:Mon(?:day)?|Tue(?:sday)?|Wed(?:nesday)?|Thu(?:rsday)?|Fri(?:day)?|Sat(?:urday)?|Sun(?:day)?)");
        let pattern = grok.compile("%{DAY} %{MONTH} %{YEAR}", false);

        let matches = pattern.match_against("Monday March 2012").expect("No matches found!");
        assert_eq!("Monday", matches.get("DAY").unwrap());
        assert_eq!("March", matches.get("MONTH").unwrap());
        assert_eq!("2012", matches.get("YEAR").unwrap());
        assert_eq!(None, matches.get("unknown"));
    }

    #[test]
    fn test_named_only() {
        let mut grok = Grok::default();
        grok.insert_definition("MAC", r"(?:%{CISCOMAC}|%{WINDOWSMAC}|%{COMMONMAC})");
        grok.insert_definition("CISCOMAC", r"(?:(?:[A-Fa-f0-9]{4}\.){2}[A-Fa-f0-9]{4})");
        grok.insert_definition("WINDOWSMAC", r"(?:(?:[A-Fa-f0-9]{2}-){5}[A-Fa-f0-9]{2})");
        grok.insert_definition("COMMONMAC", r"(?:(?:[A-Fa-f0-9]{2}:){5}[A-Fa-f0-9]{2})");
        let pattern = grok.compile("%{MAC:macaddr}", true);

        let matches = pattern.match_against("5E:FF:56:A2:AF:15").expect("No matches found!");
        assert_eq!("5E:FF:56:A2:AF:15", matches.get("macaddr").unwrap());
        assert_eq!(1, matches.len());
        let matches = pattern.match_against("hello! 5E:FF:56:A2:AF:15 what?").expect("No matches found!");
        assert_eq!("5E:FF:56:A2:AF:15", matches.get("macaddr").unwrap());
        assert_eq!(true, pattern.match_against("5E:FF").is_none());
    }
}
