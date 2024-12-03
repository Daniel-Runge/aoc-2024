use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    MUL,
    NUMBER(i32),
    LPARENS,
    RPARENS,
    COMMA,
    TRASH,
}

// fn lexer(filename: &str) -> Result<Vec<Token>, std::io::Error> {
//     let mut tokens: Vec<Token> = Vec::new();
//     let mut file = File::open(filename)?;
//     let mut contents = String::default();
//     let read = file.read_to_string(&mut contents)?;
//     let mut sum = 0;
//
//     let a = contents.chars().filter(|character| *character != '\n').collect::<Vec<char>>();
//
//     a.as_slice().windows(3).for_each(|c| {
//         println!("{:?}", c);
//         match c {
//             ['m','u','l'] => tokens.push(Token::MUL),
//             _ => return
//         }
//
//     });

// let mut buffer = [0; 128];
// loop {
//     if let read = file.read(&mut buffer)? {
//         if read == 0 { break; }
//         let text = &buffer[..read];
//         // println!("Read: {} bytes", read);
//         // println!("Read characters: {}", from_utf8(text).unwrap());
//     }
//     // break;
// }

//     println!("{:?}", tokens.len());
//
//     Ok(tokens)
// }

fn lexer(filename: &str) -> Result<Vec<Token>, std::io::Error> {
    let mut tokens = Vec::new();
    let mut file = File::open(filename)?;
    let mut contents = String::default();
    file.read_to_string(&mut contents)?;

    let mut iterator = contents.chars().peekable();

    while let Some(character) = iterator.next() {
        match character {
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

    println!("Day 3 Puzzle 1 solution: {}", result);
}

pub fn day_3_puzzle_2(filename: &str) {
    let tokens = lexer(filename).unwrap();

    println!("Day 3 Puzzle 2 solution: {}", filename);
}
