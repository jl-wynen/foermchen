use std::str::Chars;

use unic_ucd_ident::is_xid_start;

#[derive(Debug)]
pub enum Token {
    Op(char),
    Name(String),
    Number(i32),
    EOC,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    current: char,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut lexer = Self {
            chars: code.chars(),
            current: '\0',
        };
        lexer.current = match lexer.chars.next() {
            Some(c) => c,
            None => '\0',
        };
        lexer
    }

    fn seek_next(&mut self) {
        while self.current == ' ' {
            match self.chars.next() {
                Some(c) => {
                    self.current = c;
                }
                None => {
                    self.current = '\0';
                    return;
                }
            };
        }
    }

    pub fn next_char(&mut self) -> char {
        self.current = match self.chars.next() {
            Some(c) => c,
            None => '\0',
        };
        self.current
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.seek_next();
        match self.current {
            '+' | '-' | '*' => {
                let res = Token::Op(self.current);
                self.next_char();
                Ok(res)
            }
            '0'..='9' => Ok(Token::Number(self.tokenise_number())),
            '\0' => Ok(Token::EOC),
            _ => {
                if part_of_identifier(self.current) {
                    Ok(Token::Name(self.tokenise_string()))
                } else {
                    Err("Unknown token".to_owned())
                }
            }
        }
    }

    fn tokenise_string(&mut self) -> String {
        let mut res = String::new();
        while part_of_identifier(self.current) {
            res.push(self.current);
            self.next_char();
        }
        res
    }

    fn tokenise_number(&mut self) -> i32 {
        let mut res = String::new();
        loop {
            match self.current {
                '0'..='9' => {
                    res.push(self.current);
                    self.next_char();
                }
                _ => break,
            };
        }
        res.parse().unwrap()
    }
}

fn part_of_identifier(chr: char) -> bool {
    is_xid_start(chr)
}
