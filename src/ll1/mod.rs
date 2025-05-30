use std::collections::{HashMap, HashSet};

use crate::grammar::{FreeProduction, Grammar, TokenType};
use petgraph::{
    Graph,
    algo::{DfsSpace, condensation, has_path_connecting, toposort},
    graph::NodeIndex,
    visit::EdgeRef,
};

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

    let condensation_graph = condensation(first_graph, true);

    let topological_order =
        toposort(&condensation_graph, None).expect("Failed to compute topological order");

    let mut current_firsts: HashSet<usize> = HashSet::new();
    let mut prev_scc = topological_order.last().expect("Empty toposort");

    let mut dfs_space = DfsSpace::new(&condensation_graph);

    // First reachability check
    for scc in topological_order.iter().rev() {
        if !has_path_connecting(&condensation_graph, *scc, *prev_scc, Some(&mut dfs_space)) {
            current_firsts.clear();
        }

        let scc_components = condensation_graph
            .node_weight(*scc)
            .expect("Failed to get node weight");

        for node in scc_components {
            let node_firsts = first_sets.get_mut(node).expect("Failed to get first set");
            for first in node_firsts.iter() {
                if !current_firsts.contains(first) {
                    current_firsts.insert(*first);
                }
            }
        }

        for node in scc_components {
            first_sets
                .get_mut(node)
                .expect("Failed to get first set")
                .extend(current_firsts.iter().cloned());
        }

        prev_scc = scc;
    }

    first_sets.iter.map(|(key, value)| {
        let token = grammar.get_token(key).expect("Failed to get token");
        println!("First({}) = {:?}", token.content, value);
    });

    println!("First sets: {:#?}",);
    // first_graph.add_edge(
    //     *to_petgraph_index.get(&6).unwrap(),
    //     *to_petgraph_index.get(&4).unwrap(),
    //     (),
    // );

    // println!("{:#?}",);

    first_sets
}

fn get_condensation_graph(g: &Graph<usize, ()>) {}

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
