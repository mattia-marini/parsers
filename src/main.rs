mod grammar;
mod grammar_parser;
mod ll1;
mod lr0;
use grammar::{FreeProduction, Grammar, Production};
use grammar_parser::construct_grammar;
use ll1::nullabes;

fn main() {
    let grammar: Grammar<FreeProduction> =
        construct_grammar("grammar2.toml").expect("Failed to parse grammar from TOML file");

    println!("{:}", grammar);

    let nullables = nullabes(&grammar)
        .iter()
        .map(|e| grammar.get_token(e).unwrap().content.as_str())
        .collect::<Vec<_>>();

    println!("{:?}", nullables);
}
