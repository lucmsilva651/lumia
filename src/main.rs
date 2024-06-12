use std::fs;
use std::str::Chars;
use std::env;
use std::process::exit;

// Definition of possible tokens in the Lumia language
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Show,
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Equals,
    LParen,
    RParen,
    Comma,
    Eof,
}

// Lexer to transform the code into a list of tokens
struct Lexer<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(current) = self.current_char {
            match current {
                ' ' | '\t' | '\n' | '\r' => self.advance(),
                '=' => {
                    self.advance();
                    return Token::Equals;
                }
                '(' => {
                    self.advance();
                    return Token::LParen;
                }
                ')' => {
                    self.advance();
                    return Token::RParen;
                }
                ',' => {
                    self.advance();
                    return Token::Comma;
                }
                '0'..='9' | '.' => return self.number(),
                '"' => return self.string(),
                'a'..='z' | 'A'..='Z' => return self.identifier(),
                _ => self.advance(),
            }
        }
        Token::Eof
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(current) = self.current_char {
            if current.is_digit(10) || current == '.' {
                num_str.push(current);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(num_str.parse().unwrap())
    }

    fn string(&mut self) -> Token {
        let mut str_val = String::new();
        self.advance(); // Skip opening quote
        while let Some(current) = self.current_char {
            if current == '"' {
                self.advance(); // Skip closing quote
                break;
            } else {
                str_val.push(current);
                self.advance();
            }
        }
        Token::StringLiteral(str_val)
    }

    fn identifier(&mut self) -> Token {
        let mut id = String::new();
        while let Some(current) = self.current_char {
            if current.is_alphanumeric() || current == '_' {
                id.push(current);
                self.advance();
            } else {
                break;
            }
        }
        match id.as_str() {
            "show" => Token::Show,
            _ => Token::Identifier(id),
        }
    }
}

// AST structure for the Lumia language
#[derive(Debug)]
enum ASTNode {
    Show(Vec<ASTNode>),
    StringLiteral(String),
    Number(f64),
    Identifier(String),
}

// Parser to build the AST from tokens
struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            current_token: Token::Eof,
            lexer,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.get_next_token();
    }

    fn parse(&mut self) -> Option<ASTNode> {
        match &self.current_token {
            Token::Show => self.parse_show(),
            _ => None,
        }
    }

    fn parse_show(&mut self) -> Option<ASTNode> {
        self.advance(); // Skip 'show'
        self.expect(Token::LParen)?;
        let mut args = Vec::new();
        while !matches!(self.current_token, Token::RParen | Token::Eof) {
            if let Some(arg) = self.parse_expression() {
                args.push(arg);
            }
            if matches!(self.current_token, Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::RParen)?;
        Some(ASTNode::Show(args))
    }

    fn parse_expression(&mut self) -> Option<ASTNode> {
        match &self.current_token {
            Token::StringLiteral(value) => {
                let node = ASTNode::StringLiteral(value.clone());
                self.advance();
                Some(node)
            }
            Token::Number(value) => {
                let node = ASTNode::Number(*value);
                self.advance();
                Some(node)
            }
            Token::Identifier(value) => {
                let node = ASTNode::Identifier(value.clone());
                self.advance();
                Some(node)
            }
            _ => None,
        }
    }

    fn expect(&mut self, token: Token) -> Option<()> {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&token) {
            self.advance();
            Some(())
        } else {
            None
        }
    }
}

// Interpreter to execute the AST
struct Interpreter;

impl Interpreter {
    fn new() -> Self {
        Interpreter
    }

    fn interpret(&self, node: ASTNode) {
        match node {
            ASTNode::Show(args) => self.execute_show(args),
            _ => (),
        }
    }

    fn execute_show(&self, args: Vec<ASTNode>) {
        let output: Vec<String> = args.into_iter().map(|arg| match arg {
            ASTNode::StringLiteral(value) => value,
            ASTNode::Number(value) => value.to_string(),
            ASTNode::Identifier(value) => value, // Placeholder for variables
            _ => String::from(""),
        }).collect();
        println!("{}", output.join(" "));
    }
}

// Main function to execute the code
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.peb>", args[0]);
        exit(1);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);

    while let Some(ast) = parser.parse() {
        let interpreter = Interpreter::new();
        interpreter.interpret(ast);
    }
}
