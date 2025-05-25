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

pub fn nullabes(grammar: &Grammar<FreeProduction>) -> HashSet<usize> {
    let mut tmp_grammar = grammar.clone();
    let mut nullable_set: HashSet<usize> = HashSet::new();

    for prod in grammar.productions.values() {
        if prod.body.is_empty() {
            nullable_set.insert(prod.driver);
            tmp_grammar.productions.remove(&prod.id);
        }
    }

    let mut changed = true;

    while changed {
        changed = false;

        for prod in tmp_grammar.productions.values_mut() {
            prod.body
                .retain(|token_id| nullable_set.get(token_id).is_none());
            if prod.body.is_empty() {
                nullable_set.insert(prod.driver);
                changed = true;
            }
        }

        tmp_grammar
            .productions
            .retain(|_, prod| !prod.body.is_empty());
    }

    nullable_set
}
