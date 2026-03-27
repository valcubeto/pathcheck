#![allow(clippy::while_let_on_iterator)]

use crate::utils::fatal;
use std::{ iter::Peekable, str::Chars };

// todo: rewrite as a Parser struct

struct FormatParser<'a> {
    pub(self) chars: Peekable<Chars<'a>>,
    pub(self) vars: Variables,
    pub(self) lengths: &'a LongestValueSizes
}

impl<'a> FormatParser<'a> {
    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    #[must_use]
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn parse_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(letter) = self.peek() {
            if !letter.is_alphabetic() {
                break;
            }
            self.next();
            word.push(letter);
        }
        word
    }

    pub fn parse(&mut self) -> String {
        let mut parts: Vec<Component> = vec![];
        while let Some(ch) = self.next() {
            if ch == '{' {
                let interpolation = self.parse_interpolation();
                parts.push(Component::Interpolation(interpolation));
                continue
            }

            let mut acc = String::from(ch);
            while let Some(ch) = self.peek() {
                if ch == '{' {
                    break;
                }
                if ch == '\\' {
                    self.next();
                    if let Some(ch) = self.next() {
                        acc.push(ch);
                    }
                } else {
                    acc.push(ch);
                    self.next();
                }
            }
            parts.push(Component::Text(acc));
        }

        println!("{parts:#?}");
        let mut line = String::new();

        for comp in parts.iter() {
            line.push_str(match comp {
                Component::Text(text) => text,
                Component::Interpolation(interp) => interp.as_ref().map(|s| s.as_str()).unwrap_or("(no var)")
            });
        }

        line
    }

    fn parse_escaped(&mut self) -> Option<char> {
        self.next();
        self.next().map(|ch| match ch {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            ch  => ch
        })
    }

    fn parse_interpolation(&mut self) -> Option<String> {
        let mut lhs   = String::new();
        let mut value = String::new();
        let mut rhs   = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                break;
            }

            if ch == '\\' {
                self.parse_escaped()
                    .inspect(|&ch| lhs.push(ch));
                continue;
            }

            lhs.push(ch);
            self.next();
        }

        while let Some(ch) = self.peek() {
            match ch {
                '}' => break,
                letter if letter.is_alphabetic() => {
                    let word = self.parse_word();
                    let (var_value, max_length) = match word.as_str() {
                        "path" => (
                            Some(self.vars.path.clone()),
                            self.lengths.path
                        ),
                        "line" => (
                            self.vars.line.clone(),
                            self.lengths.line
                        ),
                        "status" => (
                            self.vars.status.clone(),
                            self.lengths.status
                        ),
                        "description" => (
                            self.vars.description.clone(),
                            self.lengths.description
                        ),
                        other => fatal(&format!("invalid var name {other:?}"))
                    };
                    todo!("bad handling of empty vars");
                    if let Some(var_value) = var_value && let Some(':') = self.peek() {
                        self.next();
                        match self.next() {
                            // Idk how to format this.
                            Some('<') => if let Some(ch) = self.peek() {
                                let filler = if ch == '}' { ' ' } else {
                                    self.next();
                                    ch
                                };
                                self.next();
                                value.push_str(&var_value);
                                value.push_str(&filler.to_string().repeat(max_length - var_value.len()));
                            } else {
                                fatal("unclosed formatting");
                            }
                            Some('>') => if let Some(ch) = self.peek() {
                                let filler = if ch == '}' { ' ' } else {
                                    self.next();
                                    ch
                                };
                                value.push_str(&filler.to_string().repeat(max_length - var_value.len()));
                                value.push_str(&var_value);
                            } else {
                                fatal("unclosed formatting")
                            },
                            // Some('0') => todo!("0"),
                            Some('}') => {
                                // Empty formatting.
                                break;
                            }
                            Some(other) => fatal(&format!("")),
                            None => fatal("unclosed")
                        }
                    }
                }
                other => break
            }
            self.next();
        }
        if !value.is_empty() {
            Some(format!("{lhs}{value}{rhs}"))
        } else {
            None
        }
    }

}

pub struct Variables {
    pub path: String,
    pub line: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
}

#[derive(Default)]
pub struct LongestValueSizes { 
    pub path: usize,
    pub line: usize,
    pub status: usize,
    pub description: usize,
}

#[derive(Debug)]
enum Component {
    Text(String),
    Interpolation(Option<String>)
}

pub fn format_line(fmt_string: &str, vars: Variables, lengths: &LongestValueSizes) -> String {
    let mut parser = FormatParser {
        chars: fmt_string.chars().peekable(),
        vars,
        lengths
    };

    parser.parse()
}

