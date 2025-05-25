use super::grammar::*;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
#[allow(unused)]
struct GrammarToml {
    start_symbol: usize,
    terminals: HashMap<String, String>,
    non_terminals: HashMap<String, String>,
    productions: HashMap<String, ProductionToml>,
}

#[derive(Deserialize)]
#[allow(unused)]
struct ProductionToml {
    lhs: Vec<usize>,
    rhs: Vec<usize>,
}

pub fn parse_grammar_from_toml_file(path: &str) -> Result<Grammar, &'static str> {
    let toml_str = fs::read_to_string(path).map_err(|_| "Unable to read file")?;
    let grammar_toml: GrammarToml = toml::from_str(&toml_str).map_err(|e| {
        println!("{:#?}", e);
        "TOML deserialization failed"
    })?;

    let mut grammar = Grammar::new();

    // Add terminals
    for (id, content) in grammar_toml.terminals {
        let parsed_id: usize = id.parse().map_err(|_| "Invalid terminal ID")?;
        let token = Token::new_terminal(parsed_id, &content);
        grammar.add_terminal(token)?;
    }

    // Add non-terminals
    for (id, content) in grammar_toml.non_terminals {
        let parsed_id: usize = id.parse().map_err(|_| "Invalid terminal ID")?;
        let token = Token::new_non_terminal(parsed_id, &content);
        grammar.add_non_terminal(token)?;
    }

    // Add productions
    for (id, prod_toml) in grammar_toml.productions {
        let parsed_id: usize = id.parse().map_err(|_| "Invalid terminal ID")?;
        let production = Production::new(parsed_id, prod_toml.lhs, prod_toml.rhs);
        grammar.add_production_strict(production)?;
    }

    Ok(grammar)
}
