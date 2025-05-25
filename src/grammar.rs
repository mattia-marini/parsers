#![allow(unused)]
use super::grammar_parser::{FreeProductionToml, ProductionToml};
use serde::{Deserialize, de::DeserializeOwned};
use std::{collections::HashMap, fmt::Display};

#[derive(Deserialize, Debug, Clone)]
pub struct Grammar<T>
where
    T: GrammarProduction,
{
    pub tokens: HashMap<usize, Token>,
    pub start_symbol: Option<usize>,
    pub productions: HashMap<usize, T>,
}

impl<T> Display for Grammar<T>
where
    T: GrammarProduction,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(start_symbol) = self.start_symbol {
            if let Some(token) = self.tokens.get(&start_symbol) {
                writeln!(f, "Starting symbol: {}", token.content)?;
            }
        }

        let mut sorted_productions: Vec<(&usize, &T)> = self.productions.iter().collect();
        sorted_productions.sort_by_key(|&(id, _)| *id);

        for (id, prod) in sorted_productions.iter() {
            let driver = self.get_production_driver(id).unwrap_or("Err".to_string());
            let mut body = self.get_production_body(id).unwrap_or("Err".to_string());
            if body.is_empty() {
                body = "ε".to_string();
            }

            writeln!(f, "P{}: {} -> {}", id, driver, body)?;
        }

        Ok(())
    }
}

impl<T> Grammar<T>
where
    T: GrammarProduction,
{
    pub fn new() -> Self {
        Grammar {
            start_symbol: None,
            tokens: HashMap::new(),
            productions: HashMap::new(),
        }
    }

    pub fn set_start_symbol(&mut self, id: usize) -> Result<(), &'static str> {
        if self.tokens.contains_key(&id) {
            self.start_symbol = Some(id);
            Ok(())
        } else {
            Err("Token not found in vocabulary")
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

    pub fn get_token(&self, id: &usize) -> Option<&Token> {
        self.tokens.get(id)
    }

    pub fn get_production_body(&self, id: &usize) -> Option<String> {
        match self.productions.get(id) {
            Some(prod) => {
                let mut rv = String::new();
                // println!("ids: {:?}", prod.normalized_body());
                // println!("map: {:#?}", self.tokens);
                for token_id in prod.normalized_body().iter() {
                    match self.get_token(token_id) {
                        Some(token) => {
                            // println!("TTTTT: {}, {}", token.id, token.content);
                            rv.push_str(token.content.as_str())
                        }
                        None => return None,
                    }
                }
                Some(rv)
            }
            None => None,
        }
    }

    pub fn get_production_driver(&self, id: &usize) -> Option<String> {
        match self.productions.get(id) {
            Some(prod) => {
                let mut err = false;
                let rv =
                    prod.normalized_driver()
                        .iter()
                        .fold("".to_string(), |mut acc, token_id| {
                            match self.get_token(token_id) {
                                Some(token) => {
                                    acc.push_str(token.content.as_str());
                                }
                                None => err = true,
                            };

                            acc
                        });
                if err { None } else { Some(rv) }
            }
            None => None,
        }
    }

    /// Adds the specified production. If any of the symbol involved in the productiondo not exist
    /// it creates a fresh one, assuming the driver contains only non terminals and the body only
    /// terminals
    pub fn add_production(&mut self, production: T) {
        for token_id in production.normalized_driver().iter() {
            if !self.tokens.contains_key(&token_id) {
                let new_token = Token::new_non_terminal(*token_id, &format!("NT{}", token_id));
                self.tokens.insert(*token_id, new_token);
            }
        }
        for token_id in production.normalized_body().iter() {
            if !self.tokens.contains_key(&token_id) {
                let new_token = Token::new_terminal(*token_id, &format!("NT{}", token_id));
                self.tokens.insert(*token_id, new_token);
            }
        }

        self.productions.insert(production.id(), production);
    }

    /// Checks whether each of the tokens inside the body and driver in the vocabulary. Returns
    /// error otherwise
    pub fn add_production_strict(&mut self, production: T) -> Result<(), &'static str> {
        for token_id in production.normalized_driver().iter() {
            if !self.tokens.contains_key(&token_id) {
                return Err("Token not found in vocabulary");
            }
        }
        for token_id in production.normalized_body().iter() {
            if !self.tokens.contains_key(&token_id) {
                return Err("Token not found in vocabulary");
            }
        }

        self.productions.insert(production.id(), production);
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum TokenType {
    Terminal,
    NonTerminal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Token {
    pub id: usize,
    pub content: String,
    pub token_type: TokenType,
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

pub trait GrammarProduction
where
    Self::TomlType: DeserializeOwned,
{
    type TomlType;

    fn normalized_driver<'a>(&'a self) -> &'a Vec<usize>;
    fn normalized_body<'a>(&'a self) -> &'a Vec<usize>;
    fn id(&self) -> usize;
}

pub trait FromToml<T>
where
    T: DeserializeOwned,
{
    fn from_toml(id: usize, toml: T) -> Self;
}

#[derive(Deserialize, Debug, Clone)]
pub struct Production {
    pub id: usize,
    pub driver: Vec<usize>,
    pub body: Vec<usize>,
}

impl Production {
    pub fn new(id: usize, driver: Vec<usize>, body: Vec<usize>) -> Self {
        Production { id, driver, body }
    }
}

impl GrammarProduction for Production {
    type TomlType = ProductionToml;

    fn normalized_driver<'a>(&'a self) -> &'a Vec<usize> {
        &self.driver
    }

    fn normalized_body<'a>(&'a self) -> &'a Vec<usize> {
        &self.body
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl FromToml<ProductionToml> for Production {
    fn from_toml(id: usize, toml: ProductionToml) -> Self {
        Production {
            id,
            driver: toml.lhs,
            body: toml.rhs,
        }
    }
}

impl Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let driver = self.driver.iter().fold("".to_string(), |mut acc, id| {
            acc.push_str(id.to_string().as_str());
            acc
        });

        let body = self.body.iter().fold("".to_string(), |mut acc, id| {
            acc.push_str(id.to_string().as_str());
            acc
        });

        write!(f, "{} -> {}", driver, body)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FreeProduction {
    pub id: usize,
    pub driver: usize,
    pub body: Vec<usize>,

    normalized_driver: Vec<usize>,
}

impl FreeProduction {
    pub fn new(id: usize, driver: usize, body: Vec<usize>) -> Self {
        FreeProduction {
            id,
            driver,
            body,
            normalized_driver: vec![driver],
        }
    }
}

impl GrammarProduction for FreeProduction {
    type TomlType = FreeProductionToml;

    fn normalized_driver<'a>(&'a self) -> &'a Vec<usize> {
        &self.normalized_driver
    }

    fn normalized_body<'a>(&'a self) -> &'a Vec<usize> {
        &self.body
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl FromToml<FreeProductionToml> for FreeProduction {
    fn from_toml(id: usize, toml: FreeProductionToml) -> Self {
        FreeProduction {
            id,
            driver: toml.lhs,
            body: toml.rhs,
            normalized_driver: vec![toml.lhs],
        }
    }
}

impl Display for FreeProduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let driver = self.driver.to_string();

        let body = self.body.iter().fold("".to_string(), |mut acc, id| {
            acc.push_str(id.to_string().as_str());
            acc
        });

        write!(f, "{} -> {}", driver, body)
    }
}
