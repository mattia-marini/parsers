use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Grammar {
    tokens: HashMap<usize, Token>,
    start_symbol: Option<usize>,
    productions: HashMap<usize, Production>,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar {
            start_symbol: None,
            tokens: HashMap::new(),
            productions: HashMap::new(),
        }
    }

    pub fn add_terminal(&mut self, token: Token) -> Result<(), &'static str> {
        if self.tokens.contains_key(&token.id) {
            Err("Id already existing")
        } else {
            self.tokens.insert(token.id, token);
            Ok(())
        }
    }

    pub fn add_non_terminal(&mut self, token: Token) -> Result<(), &'static str> {
        if self.tokens.contains_key(&token.id) {
            Err("Id already existing")
        } else {
            self.tokens.insert(token.id, token);
            Ok(())
        }
    }

    /// Checks whether each of the tokens inside the body and driver in the vocabulary. Returns
    /// error otherwise
    pub fn add_production_strict(&mut self, production: Production) -> Result<(), &'static str> {
        for token_id in production.driver.iter() {
            if !self.tokens.contains_key(&token_id) {
                return Err("Token not found in vocabulary");
            }
        }
        for token_id in production.body.iter() {
            if !self.tokens.contains_key(&token_id) {
                return Err("Token not found in vocabulary");
            }
        }

        self.productions.insert(production.id, production);
        Ok(())
    }

    /// Adds the specified production. If any of the symbol involved in the productiondo not exist
    /// it creates a fresh one, assuming the driver contains only non terminals and the body only
    /// terminals
    pub fn add_production(&mut self, production: Production) {
        for token_id in production.driver.iter() {
            if !self.tokens.contains_key(&token_id) {
                let new_token = Token::new_non_terminal(*token_id, &format!("NT{}", token_id));
                self.tokens.insert(*token_id, new_token);
            }
        }
        for token_id in production.body.iter() {
            if !self.tokens.contains_key(&token_id) {
                let new_token = Token::new_terminal(*token_id, &format!("NT{}", token_id));
                self.tokens.insert(*token_id, new_token);
            }
        }

        self.productions.insert(production.id, production);
    }
}

#[derive(Deserialize)]
pub enum TokenType {
    Terminal,
    NonTerminal,
}

#[derive(Deserialize)]
pub struct Token {
    id: usize,
    content: String,
    token_type: TokenType,
}

impl Token {
    /// Creates new token based on the case of the specified char
    pub fn new(id: usize, content: &str) -> Self {
        if content.chars().next().unwrap().is_uppercase() {
            Token {
                id,
                content: String::from(content),
                token_type: TokenType::NonTerminal,
            }
        } else {
            Token {
                id,
                content: String::from(content),
                token_type: TokenType::NonTerminal,
            }
        }
    }

    pub fn new_terminal(id: usize, content: &str) -> Self {
        Token {
            id,
            content: String::from(content),
            token_type: TokenType::Terminal,
        }
    }
    pub fn new_non_terminal(id: usize, content: &str) -> Self {
        Token {
            id,
            content: String::from(content),
            token_type: TokenType::NonTerminal,
        }
    }
}

#[derive(Deserialize)]
pub struct Production {
    id: usize,
    driver: Vec<usize>,
    body: Vec<usize>,
}

impl Production {
    pub fn new(id: usize, driver: Vec<usize>, body: Vec<usize>) -> Self {
        Production { id, driver, body }
    }
}
