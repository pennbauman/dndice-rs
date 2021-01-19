// parse - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::fmt;


/// Error from failed parsing of dice from a string
#[derive(Debug)]
pub enum DiceParseError {
    InvalidNumber(String),
    InvalidDie(String),
    InvalidChar(char),
    InvalidMath(String),
}
impl fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber(s) => write!(f, "Invalid number '{}'", s),
            Self::InvalidDie(s) => write!(f, "Invalid die '{}'", s),
            Self::InvalidChar(s) => write!(f, "Invalid character '{}'", s),
            Self::InvalidMath(s) => write!(f, "Invalid expression '{}'", s),
        }
    }
}

// Type of Dice Set
#[derive(Debug, Copy, Clone)]
pub enum ParseKind {
    Sum,
    Mult,
    Die,
    Const,
}

// State of Parser
#[derive(Debug)]
pub struct ParseState {
    split: Vec<(char, String)>,
    kind: ParseKind,
    history: String,
    current: String,
    breakchar: char,
    previous: char,
}
impl ParseState {
    pub fn new() -> Self {
        Self {
            split: vec![],
            kind: ParseKind::Const,
            history: String::new(),
            current: String::new(),
            breakchar: '+',
            previous: ' '
        }
    }

    pub fn splits(&self) -> &Vec<(char, String)> {
        &self.split
    }
    pub fn kind(&self) -> ParseKind {
        self.kind
    }

    pub fn parse_from(text: &str) -> Result<Self, char> {
        //println!("{}", text);
        let mut state = Self::new();
        for c in text.chars() {
            match state.next(c) {
                Ok(_) => (),
                Err(_) => return Err(c),
            }
        }
        state.close();
        return Ok(state);
    }
    fn next(&mut self, c: char) -> Result<(), ()> {
        if c == '+' || (c == '-' && self.previous != '*') {
            match self.kind {
                ParseKind::Sum => {
                    self.split.push((self.breakchar, self.current.to_string()));
                    self.current = String::new();
                    self.breakchar = c;
                },
                _ => {
                    if self.history != "" {
                        self.split = vec![('+', self.history.to_string())];
                    }
                    self.current = String::new();
                    self.breakchar = c;
                    self.kind = ParseKind::Sum;
                },
            };
        } else if c == '*' || c == 'x' {
            match self.kind {
                ParseKind::Sum => self.current.push(c),
                ParseKind::Mult => {
                    self.split.push(('*', self.current.to_string()));
                    self.current = String::new();
                    self.breakchar = '*';
                },
                _ => {
                    self.split = vec![('*', self.history.to_string())];
                    self.current = String::new();
                    self.breakchar = '*';
                    self.kind = ParseKind::Mult;
                },
            };
        } else if c == 'd' || c == 'D' {
            match self.kind {
                ParseKind::Sum => self.current.push('d'),
                ParseKind::Mult => self.current.push('d'),
                ParseKind::Die => {
                    self.split.push(('d', self.current.to_string()));
                    self.current = String::new();
                    self.breakchar = 'd';
                },
                ParseKind::Const => {
                    self.split = vec![('d', self.history.to_string())];
                    self.current = String::new();
                    self.breakchar = 'd';
                    self.kind = ParseKind::Die;
                },
            };
        } else if c.is_digit(10) || c == '-' {
            self.current.push(c);
        } else if c.is_whitespace() {
            ();
        } else {
            return Err(());
        }
        if !c.is_whitespace() {
            self.previous = c;
            self.history.push(c);
        }
        return Ok(());
    }
    fn close(&mut self) {
        self.split.push((self.breakchar, self.current.to_string()));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state_new() {
        let ps = ParseState::new();
        assert!(ps.split.len() == 0);
        assert!(ps.history == "");
        assert!(ps.current == "");
        assert!(ps.breakchar == '+');
        match ps.kind {
            ParseKind::Const => (),
            _ => panic!(),
        }
    }
    #[test]
    fn test_parse_state_next_mult() {
        let mut ps = ParseState {
            split: vec![],
            kind: ParseKind::Const,
            history: String::from("history"),
            current: String::from("current"),
            breakchar: '+',
            previous: ' ',
        };
        ps.next('*').unwrap();
        assert!(ps.split[0].0 == '*');
        assert!(ps.split[0].1 == "history");
        match ps.kind {
            ParseKind::Mult => (),
            _ => panic!(),
        }
    }
    #[test]
    fn test_parse_state_next_sum() {
        let mut ps = ParseState {
            split: vec![('-', String::from("4"))],
            kind: ParseKind::Sum,
            history: String::from("x"),
            current: String::from("3"),
            breakchar: '+',
            previous: ' ',
        };
        ps.next('+').unwrap();
        assert!(ps.split.len() == 2);
        assert!(ps.split[1].0 == '+');
        assert!(ps.split[1].1 == "3");
    }
    #[test]
    fn test_parse_state_next_die() {
        let mut ps = ParseState {
            split: vec![('+', String::from("1d4"))],
            kind: ParseKind::Sum,
            history: String::from("1d4+"),
            current: String::from("2"),
            breakchar: '+',
            previous: ' ',
        };
        ps.next('D').unwrap();
        assert!(ps.split.len() == 1);
        assert!(ps.current == "2d");
    }
    #[test]
    fn test_parse_state_next_digit() {
        let mut ps = ParseState::new();
        ps.current = String::from("1234");
        ps.next('5').unwrap();
        assert!(ps.current == "12345");
    }
    #[test]
    fn test_parse_state_next_space() {
        let mut ps = ParseState::new();
        ps.current = String::from("1234");
        ps.next(' ').unwrap();
        assert!(ps.current == "1234");
    }
    #[test]
    fn test_parse_state_next_error() {
        let mut ps = ParseState::new();
        assert!(ps.next('g').is_err());
    }
    #[test]
    fn test_parse_state_close() {
        let mut ps = ParseState {
            split: vec![('1', String::from("test1"))],
            kind: ParseKind::Const,
            history: String::new(),
            current: String::from("test2"),
            breakchar: '2',
            previous: ' ',
        };
        ps.close();
        assert!(ps.split.len() == 2);
        assert!(ps.split[1].0 == '2');
        assert!(ps.split[1].1 == "test2");
    }
}
