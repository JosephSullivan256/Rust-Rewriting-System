use regex::Regex;

#[derive(Debug)]
pub enum Token {
    //Number(f64),
    Symbol(String),
    Left,
    Right,
    Comma,
    Error,
    End
}

impl Token {
    pub fn iter_from_exp(expression : &str) -> impl Iterator<Item = Token> + '_ {
        lazy_static! {
            //static ref RE: Regex = Regex::new(r"(?P<number>\d*\.?\d+)|(?P<symbol>[\w\+\-\*/\|=\&%\^]+)|(?P<left>\()|(?P<right>\))|(?P<comma>,)").unwrap();
            static ref RE: Regex = Regex::new(r"(?P<symbol>[\w\+\-\*/\|=\&%\^]+|(\d*\.?\d)+)|(?P<left>\()|(?P<right>\))|(?P<comma>,)").unwrap();
        }
        RE.captures_iter(expression)
            .map(|x| {
                //if let Some(n) = x.name("number") {
                //    return Token::Number(n.as_str().parse().unwrap());
                //}
                if let Some(n) = x.name("symbol") {
                    return Token::Symbol(String::from(n.as_str()));
                }
                if let Some(_n) = x.name("left") {
                    return Token::Left;
                }
                if let Some(_n) = x.name("right") {
                    return Token::Right;
                }
                if let Some(_n) = x.name("comma") {
                    return Token::Comma;
                }
                return Token::Error;
            })
    }
}