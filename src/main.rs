mod grammar;
mod grammar_parser;
use grammar_parser::parse_grammar_from_toml_file;

fn main() {
    let grammar = parse_grammar_from_toml_file("grammar.toml")
        .expect("Failed to parse grammar from TOML file");
    println!("{:}", grammar);
}
