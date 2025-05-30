use std::collections::{HashMap, HashSet};

use crate::grammar::{FreeProduction, Grammar, TokenType};
use petgraph::{Graph, graph::NodeIndex, visit::EdgeRef};

/// 1. Compute "basic firsts"
/// 2. Create dependencies graph. There is a node for each driver and for each productions
/// there is an edge from A to B iif first(A) = <something> + first(B)
/// 3. Compute condensation graph and topological sort
/// 4. For each SCC compute the set of different first that appear on any node and assign that to
///    each node
/// 5. Repear for each SCC, keeping track of all the previously added entries

pub fn first(grammar: &Grammar<FreeProduction>) -> HashMap<usize, HashSet<usize>> {
    let mut first_sets: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut to_petgraph_index: HashMap<usize, NodeIndex> = HashMap::new();

    let nullable_set = nullabes(grammar);

    //for each node store the corresponding non terminal id
    let mut first_graph = Graph::<usize, ()>::new();
    for (token_id, token) in grammar.tokens.iter() {
        if let TokenType::NonTerminal = token.token_type {
            let node_index = first_graph.add_node(token.id);
            to_petgraph_index.insert(token.id, node_index);
            first_sets.insert(token.id, HashSet::new());
        }
    }

    grammar.print_dbg();

    // println!("First graph: {:?}", first_graph);
    // Computing basic firts

    for prod in grammar.productions.values() {
        for body_token_id in &prod.body {
            let body_token = grammar.get_token(body_token_id).unwrap();
            match body_token.token_type {
                TokenType::Terminal => {
                    first_sets
                        .get_mut(&prod.driver)
                        .unwrap()
                        .insert(body_token.id);
                    break;
                }
                TokenType::NonTerminal => {
                    first_graph.add_edge(
                        *to_petgraph_index.get(&prod.driver).unwrap(),
                        *to_petgraph_index.get(&body_token.id).unwrap(),
                        (),
                    );
                }
            }

            if !nullable_set.contains(body_token_id) {
                break;
            }
        }
    }

    let edges: Vec<(String, String)> = first_graph
        .edge_references()
        .map(|e| {
            let from = e.source();
            let to = e.target();
            let from_token = grammar
                .get_token(first_graph.node_weight(from).unwrap())
                .unwrap();

            let to_token = grammar
                .get_token(first_graph.node_weight(to).unwrap())
                .unwrap();
            (from_token.content.clone(), to_token.content.clone())
        })
        .collect();
    for (from, to) in edges {
        println!("edge {} -> {}", from, to);
    }


     

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
