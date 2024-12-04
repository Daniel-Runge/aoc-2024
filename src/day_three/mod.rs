use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    DO,
    DONT,
    MUL,
    NUMBER(i32),
    LPARENS,
    RPARENS,
    COMMA,
    TRASH,
}

fn lexer(filename: &str) -> Result<Vec<Token>, std::io::Error> {
    let mut tokens = Vec::new();
    let mut file = File::open(filename)?;
    let mut contents = String::default();
    file.read_to_string(&mut contents)?;
    let mut iterator = contents.chars().peekable();

    while let Some(character) = iterator.next() {
        match character {
            'd' => process_do_and_dont(&mut iterator, &mut tokens),
            'm' => process_mul(&mut tokens, &mut iterator),
            '(' => tokens.push(Token::LPARENS),
            ')' => tokens.push(Token::RPARENS),
            ',' => tokens.push(Token::COMMA),
            number if character.is_numeric() => {
                let total = String::from(number);
                look_ahead_number(&mut tokens, &mut iterator, total);
            }
            _ => tokens.push(Token::TRASH),
        }
    }

    Ok(tokens)
}

fn process_mul(tokens: &mut Vec<Token>, iterator: &mut Peekable<Chars>) {
    if let Some('u') = iterator.peek() {
        iterator.next();
        if let Some('l') = iterator.peek() {
            iterator.next();
            tokens.push(Token::MUL);
        } else {
            tokens.push(Token::TRASH)
        }
    } else {
        tokens.push(Token::TRASH)
    }
}

fn look_ahead_number(tokens: &mut Vec<Token>, iterator: &mut Peekable<Chars>, mut total: String) {
    if let Some(character) = iterator.next() {
        if character.is_numeric() {
            total.push(character);
            if let Some(next_char) = iterator.next() {
                process_numeric_character(tokens, next_char, total);
            } else {
                tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
            }
        } else {
            process_numeric_character(tokens, character, total);
        }
    }
}

fn process_do_and_dont(iterator: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    if matches!(iterator.peek(), Some('o')) {
        iterator.next();
        if matches!(iterator.peek(), Some('n')) {
            iterator.next();
            if matches!(iterator.peek(), Some('\'')) {
                iterator.next();
                if matches!(iterator.peek(), Some('t')) {
                    iterator.next();
                    tokens.push(Token::DONT);
                    return;
                }
            } else {
                tokens.push(Token::TRASH);
            }
        } else {
            tokens.push(Token::DO);
        }
    }
}

fn process_numeric_character(tokens: &mut Vec<Token>, character: char, mut total: String) {
    let number_token = Token::NUMBER(total.parse::<i32>().unwrap());
    match character {
        '(' => {
            tokens.push(number_token);
            tokens.push(Token::LPARENS);
        }
        ')' => {
            tokens.push(number_token);
            tokens.push(Token::RPARENS);
        }
        ',' => {
            tokens.push(number_token);
            tokens.push(Token::COMMA);
        }
        number if number.is_numeric() => {
            total.push(number);
            tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
        }
        _ => {
            tokens.push(number_token);
            tokens.push(Token::TRASH);
        }
    }
}

pub fn day_3_puzzle_1(filename: &str) {
    let tokens = lexer(filename).unwrap();
    let mut result = 0;
    for token in tokens.windows(6) {
        match token {
            [Token::MUL, Token::LPARENS, Token::NUMBER(x), Token::COMMA, Token::NUMBER(y), Token::RPARENS] => {
                result += x * y
            }
            _ => continue,
        }
    }

    println!("Day 3 Puzzle 1 solution: {result}");
}

pub fn day_3_puzzle_2(filename: &str) {
    let tokens = lexer(filename).unwrap();
    let mut enable_mul = true;
    let mut result = 0;
    for token in tokens.windows(6) {
        match token {
            [Token::DO, Token::LPARENS, Token::RPARENS, _, _, _] => enable_mul = true,
            [Token::DONT, Token::LPARENS, Token::RPARENS, _, _, _] => enable_mul = false,
            [Token::MUL, Token::LPARENS, Token::NUMBER(x), Token::COMMA, Token::NUMBER(y), Token::RPARENS]
                if enable_mul =>
            {
                result += x * y
            }
            _ => continue,
        }
    }

    println!("Day 3 Puzzle 2 solution: {result}");
}
