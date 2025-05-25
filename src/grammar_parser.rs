use super::grammar::*;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct GrammarToml<T> {
    start_symbol: usize,
    terminals: HashMap<String, String>,
    non_terminals: HashMap<String, String>,
    productions: HashMap<String, T>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ProductionToml {
    pub lhs: Vec<usize>,
    pub rhs: Vec<usize>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct FreeProductionToml {
    pub lhs: usize,
    pub rhs: Vec<usize>,
}

// Optional trait for semantic grouping (not used by Serde)
#[allow(dead_code)]
trait GrammarProductionToml {}
impl GrammarProductionToml for ProductionToml {}
impl GrammarProductionToml for FreeProductionToml {}

fn read_grammar_toml<T>(path: &str) -> Result<GrammarToml<T>, &'static str>
where
    T: for<'de> Deserialize<'de>, // less restrictive than 'static
{
    let toml_str = fs::read_to_string(path).map_err(|_| "Unable to read file")?;

    toml::from_str::<GrammarToml<T>>(&toml_str).map_err(|e| {
        println!("{:#?}", e);
        "TOML deserialization failed"
    })
}

pub fn construct_grammar<T>(path: &str) -> Result<Grammar<T>, &'static str>
where
    T: GrammarProduction,
    T: FromToml<T::TomlType> + std::fmt::Display,
    T::TomlType: std::fmt::Debug,
{
    let grammar_toml: GrammarToml<T::TomlType> =
        read_grammar_toml(path).map_err(|_| "Failed to read grammar from TOML file")?;

    // println!("{:#?}", grammar_toml);
    let mut grammar: Grammar<T> = Grammar::new();
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
        let production = T::from_toml(parsed_id, prod_toml);

        // println!("{:}", production);
        grammar.add_production_strict(production)?;
    }

    grammar
        .set_start_symbol(grammar_toml.start_symbol)
        .map_err(|_| "Failed to set start symbol")?;

    Ok(grammar)
}
