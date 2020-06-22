use regex;
use super::*;
use std::convert::TryInto;

pub trait RegexLexeme {
    fn token_type() -> token::TokenType;
    fn expression() -> &'static str;
    fn recognize_raw_match(input : &str) -> Result<regex::Match, &'static str> {

        // early return, in case of invalid regex
        if let Err(e) = regex::Regex::new(Self::expression()) {
            return Err("Cannot compile regex.")
        }

        let matcher = regex::Regex::new(Self::expression()).unwrap();
        match matcher.find(input) {
            Some(position) => {
                if (position.start() > 0) {
                    Err("There is a match, but it is far away.")
                } else {
                    Ok (position)
                }
            },
            None => {
                Err("No match at all.")
            }
        }
    }
}

pub trait SelfContained {}

impl<T> lexeme::Lexeme for T
where
    T : RegexLexeme + SelfContained,
{
    // using err here because it allows to express the reason for failing
    fn recognize(input : &str) -> Result<token::Token, &'static str> {
        match <Self as RegexLexeme>::recognize_raw_match(input) {
            Ok(position) => {
                Ok(
                    token::Token {
                        span : vec![
                            (position.end() -
                             position.start())
                                .try_into()
                                .unwrap()],
                        token_type : Self::token_type(),
                    }
                )
            },
            Err(reason) => {
                Err(reason)
            }
        }
    }
}
