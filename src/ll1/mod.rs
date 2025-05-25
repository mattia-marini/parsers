use std::collections::{HashMap, HashSet};

use crate::grammar::{FreeProduction, Grammar};

pub fn first(grammar: &Grammar<FreeProduction>) -> HashMap<usize, HashSet<usize>> {
    let mut first_sets: HashMap<usize, HashSet<usize>> = HashMap::new();

    for token in grammar.tokens.values() {
        first_sets.insert(token.id, HashSet::new());
    }

    // for token in grammar.tokens.values() {
    //     compute_first(token.id, &grammar, &mut first_sets);
    // }

    first_sets
}

pub fn nullabes(grammar: &Grammar<FreeProduction>) -> HashMap<usize, bool> {
    let mut nullable_sets: HashMap<usize, bool> = HashMap::new();
    // let mut last_nullables = vec![];

    todo!()
    // for prod in grammar.productions.values() {
    //     if prod.body.is_empty() {
    //         nullable_sets.insert(prod.driver, true);
    //         prod.
    //     } else {
    //         for token in &prod.body {
    //             nullable_sets.insert(*token, false);
    //         }
    //     }
    // }
    //
    // for token in grammar.tokens.values() {
    //     compute_nullable(token.id, &grammar, &mut nullable_sets);
    // }

    // nullable_sets
}
