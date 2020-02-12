use super::token::Token;
use std::mem;
use std::fmt;
use regex::Regex;

pub struct Parser {
    constant_regex: Option<Regex>
}

impl Parser {
    pub fn new() -> Parser{
        Parser {constant_regex: None}
    }
    pub fn new_with_regex(constant_regex : Regex) -> Parser{
        Parser {constant_regex: Some(constant_regex)}
    }
    pub fn new_with_regex_str(constant_regex : &str) -> Parser{
        Parser {constant_regex: Some(Regex::new(constant_regex).unwrap()) }
    }
    
    pub fn parse(& self, line : &str) -> ASTNode {
        let token_iter = Token::iter_from_exp(line);
        self.parse_token(token_iter)
    }

    pub fn parse_token(& self, token_iter : impl Iterator<Item=Token>) -> ASTNode {
        if let Some(re) = &self.constant_regex {
            ParseTask::new_with_constant(token_iter, re).parse()
        } else {
            ParseTask::new(token_iter).parse()
        }
    }
}

pub struct ParseTask<T : Iterator<Item=Token>> {
    constant_regex: Option<Regex>,
    token_iter : T,
    current : Token
}

impl<T : Iterator<Item=Token>> ParseTask<T> {
    pub fn new(mut token_iter : T) -> ParseTask<T>{
        let token = token_iter.next();
        ParseTask {
            constant_regex: None,
            token_iter,
            current: token.unwrap()
        }
    }

    pub fn new_with_constant(mut token_iter : T, constant_regex : &Regex) -> ParseTask<T>{
        let token = token_iter.next();
        ParseTask {
            constant_regex: Some(constant_regex.clone()),
            token_iter,
            current: token.unwrap()
        }
    }

    fn next(&mut self){
        self.current = if let Some(t) = self.token_iter.next() {
            t
        } else {
            Token::End
        }
    }

    fn accept(&mut self, t: Token) -> bool {
        let accepted = mem::discriminant(&self.current) == mem::discriminant(&t);
        if accepted { self.next(); }
        accepted
    }

    fn expect_symbol(&mut self) -> String {
        let sym = if let Token::Symbol(s) = &self.current {
            String::from(s)
        } else {
            panic!("invalid syntax")
        };
        self.next();
        return sym;
    }

    fn expect(& mut self, t : Token) {
        if !self.accept(t) { panic!("invalid syntax"); }
    }

    fn expression(& mut self) -> ASTNode {
        let sym = self.expect_symbol();
        if !self.accept(Token::Left) {
            if let Some(re) = &self.constant_regex {
                if re.is_match(&sym) {
                    return ASTNode::Constant(sym);
                }
            }
            return ASTNode::Symbol(sym);
        }
        
        let mut exps = Vec::new();
        exps.push(self.expression());
        while !self.accept(Token::Right) {
            self.expect(Token::Comma);
            exps.push(self.expression());
        }
        
        return ASTNode::Function(sym,exps);
    }

    // EBNF
    // expression = value | symbol left expression { comma expression } right;
    // value = number | symbol;
    pub fn parse(& mut self) -> ASTNode {
        self.expression()
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum ASTNode {
    Function(String,Vec<ASTNode>),
    Symbol(String),
    Constant(String),
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::Function(s,children) =>{
                write!(f,"{}(",s)?;
                for node in children.iter().enumerate() {
                    write!(f,"{}",node.1)?;
                    if node.0 < children.len()-1 {
                        write!(f,",")?;
                    }
                }
                write!(f,")")
            },
            ASTNode::Symbol(s) => write!(f, "{}", s),
            ASTNode::Constant(c) => write!(f, "{}", c),
        }
    }
}