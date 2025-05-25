mod grammar;
mod grammar_parser;
mod ll1;
mod lr0;
use grammar::{Grammar, Production};
use grammar_parser::construct_grammar;

fn main() {
    let grammar: Grammar<Production> =
        construct_grammar("grammar.toml").expect("Failed to parse grammar from TOML file");

    println!("{:}", grammar);
}
