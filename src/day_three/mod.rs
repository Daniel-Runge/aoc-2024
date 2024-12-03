use std::fs::File;
use std::io::Read;

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
            'd' => {
                if let Some('o') = iterator.peek() {
                    iterator.next();
                    if let Some('n') = iterator.peek() {
                        iterator.next();
                        if let Some('\'') = iterator.peek() {
                            iterator.next();
                            if let Some('t') = iterator.peek() {
                                iterator.next();
                                tokens.push(Token::DONT);
                            } else {
                                tokens.push(Token::TRASH);
                            }
                        } else {
                            tokens.push(Token::TRASH);
                        }
                    } else {
                        tokens.push(Token::DO);
                    }
                } else {
                    tokens.push(Token::TRASH);
                }
            }
            'm' => {
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
            '(' => tokens.push(Token::LPARENS),
            ')' => tokens.push(Token::RPARENS),
            ',' => tokens.push(Token::COMMA),
            number if character.is_numeric() => {
                let mut total = String::from(number);
                if let Some(character) = iterator.next() {
                    match character {
                        number if number.is_numeric() => {
                            total = total + &number.to_string();
                            if let Some(character) = iterator.next() {
                                match character {
                                    number if number.is_numeric() => {
                                        total = total + &number.to_string();
                                        tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                                    }
                                    '(' => {
                                        tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                                        tokens.push(Token::LPARENS);
                                    }
                                    ')' => {
                                        tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                                        tokens.push(Token::RPARENS);
                                    }
                                    ',' => {
                                        tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                                        tokens.push(Token::COMMA);
                                    }
                                    _ => {
                                        tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                                        tokens.push(Token::TRASH);
                                    }
                                }
                            }
                        }
                        '(' => {
                            tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                            tokens.push(Token::LPARENS);
                        }
                        ')' => {
                            tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                            tokens.push(Token::RPARENS);
                        }
                        ',' => {
                            tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                            tokens.push(Token::COMMA);
                        }
                        _ => {
                            tokens.push(Token::NUMBER(total.parse::<i32>().unwrap()));
                            tokens.push(Token::TRASH);
                        }
                    }
                }
            }
            _ => tokens.push(Token::TRASH),
        }
    }

    Ok(tokens)
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
            _ => enable_mul = enable_mul,
        }

        match token {
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
