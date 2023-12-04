use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

// This is nice Forth introduction: https://skilldrick.github.io/easyforth/

#[derive(Copy, Clone, Debug)]
enum Word {
    ADD,
    MINUS,
    MULTIPLY,
    DIVIDE,
    DUP,
    DROP,
    SWAP,
    OVER,
    LITERAL(Value),
    WORD(u32),
}

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, u32>,
    unique_words: HashMap<u32, Vec<Word>>,
    next_unique_name: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn take_untill<F>(lexems: &[String], predicate: F) -> Vec<String>
where
    F: Fn(&String) -> bool,
{
    let mut ret = Vec::new();
    for e in lexems {
        let should_stop = predicate(&e);
        if should_stop {
            break;
        }
        ret.push(e.clone());
    }
    ret
}

impl Forth {
    pub fn new() -> Forth {
        let mut words = HashMap::new();
        let mut unique_words: HashMap<u32, Vec<Word>> = HashMap::new();
        let mut next_unique_name: u32 = 0;
        let mut add_word = |name: &str, action: Word| {
            unique_words.insert(next_unique_name, vec![action]);
            words.insert(name.to_string(), next_unique_name);
            next_unique_name += 1;
        };
        use Word::*;
        add_word("*", MULTIPLY);
        add_word("/", DIVIDE);
        add_word("+", ADD);
        add_word("-", MINUS);
        add_word("dup", DUP);
        add_word("drop", DROP);
        add_word("swap", SWAP);
        add_word("over", OVER);
        Forth {
            stack: Vec::new(),
            words,
            unique_words,
            next_unique_name,
        }
    }

    fn add_word(&mut self, name: &String, words: &Vec<Word>) {
        self.unique_words
            .insert(self.next_unique_name, words.clone());
        self.words.insert(name.clone(), self.next_unique_name);
        self.next_unique_name += 1;
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let lower = input.to_lowercase();

        let lexems = lower
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        if lexems.len() == 0 {
            return Ok(());
        }

        let mut lexems: &[String] = lexems.as_slice();

        while !lexems.is_empty() {
            if lexems[0] == ":" {
                let sub_lexems = take_untill(&lexems, |s| s == ";");
                if sub_lexems.len() >= 3 && lexems.len() != 3 {
                    let name = &sub_lexems[1];
                    if name.as_bytes()[0] > b'0' && name.as_bytes()[0] < b'9' {
                        return Err(Error::InvalidWord);
                    }

                    let words = self.parse(&sub_lexems[2..])?;
                    self.add_word(&name, &words);

                    lexems = &lexems[sub_lexems.len() + 1..];
                } else {
                    return Err(Error::InvalidWord);
                }
            } else {
                let sub_lexems = take_untill(&lexems, |s| s == ":");
                if !sub_lexems.is_empty() {
                    let words = self.parse(sub_lexems.as_slice())?;
                    self.exec(&words)?;
                }
                lexems = &lexems[sub_lexems.len()..];
            }
        }
        Ok(())
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn exec(&mut self, words: &Vec<Word>) -> Result {
        let mut words: VecDeque<Word> = words.iter().map(|w| *w).collect();

        while let Some(w) = words.pop_front() {
            match w {
                Word::ADD => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.stack.push(left + right);
                }
                Word::MINUS => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.stack.push(left - right);
                }
                Word::MULTIPLY => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.stack.push(left * right);
                }
                Word::DIVIDE => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(left / right);
                }
                Word::DUP => {
                    let elem = self.stack.last().ok_or(Error::StackUnderflow)?;
                    self.stack.push(*elem);
                }
                Word::DROP => {
                    self.pop()?;
                }
                Word::SWAP => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.stack.push(right);
                    self.stack.push(left);
                }
                Word::OVER => {
                    let len = self.stack.len();
                    if len < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let elem = self
                        .stack
                        .get(len - 2)
                        .ok_or(Error::StackUnderflow)?;
                    self.stack.push(*elem);
                }
                Word::LITERAL(v) => self.stack.push(v),
                Word::WORD(w) => {
                    let sub_words = self.unique_words.get(&w).ok_or(Error::InvalidWord)?;
                    for sw in sub_words.iter().rev() {
                        words.push_front(*sw);
                    }
                }
            }
        }
        Ok(())
    }

    fn parse(&mut self, lexems: &[String]) -> std::result::Result<Vec<Word>, Error> {
        let mut words: Vec<Word> = Vec::new();
        for l in lexems {
            match l.parse::<Value>() {
                Ok(lit) => words.push(Word::LITERAL(lit)),
                Err(_) => {
                    words.push(Word::WORD(*self.words.get(l).ok_or(Error::UnknownWord)?));
                }
            }
        }
        Ok(words)
    }
}
