// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{GraphBase, IntoNodeIdentifiers, NodeIndexable};
use petgraph::{Directed, Undirected};
use std::collections::{HashMap, HashSet};

pub trait Intersection: GraphBase {
    fn intersection(&self, graph: &Self) -> Self;
}

impl<N, E> Intersection for Graph<N, E, Undirected>
where
    N: Clone,
    E: Clone,
{
    fn intersection(&self, graph: &Self) -> Self {
        let mut graph_r: Graph<N, E, Undirected> = self.clone();

        let self_indeces: HashSet<_> = self.node_identifiers().map(|n| self.to_index(n)).collect();
        let graph_indeces: HashSet<_> = graph
            .node_identifiers()
            .map(|n| graph.to_index(n))
            .collect();
        let remove_indeces = &self_indeces - &graph_indeces;
        remove_indeces.iter().for_each(|&n| {
            graph_r.remove_node(NodeIndex::new(n));
        });

        let mut edges_count: HashMap<(NodeIndex, NodeIndex), usize> = HashMap::new();
        for e in graph.edge_indices() {
            if let Some((n, m)) = graph.edge_endpoints(e) {
                let (n1, m1) = if n < m {(n, m)} else {(m, n)};
                *edges_count.entry((n1, m1)).or_insert(0) += 1;
            }
        }

        for e in graph_r.edge_indices().rev() {
            if let Some((n, m)) = graph_r.edge_endpoints(e) {
                let (n1, m1) = if n < m {(n, m)} else {(m, n)};
                if let Some(c) = edges_count.get(&(n1, m1)) {
                    if c == &0 {
                        graph_r.remove_edge(e);
                    } else {
                        *edges_count.entry((n1, m1)).or_insert(0) -= 1;
                    }
                } else {
                    graph_r.remove_edge(e);
                }
            }
        }

        return graph_r;
    }
}

impl<N, E> Intersection for Graph<N, E, Directed>
where
    N: Clone,
    E: Clone,
{
    fn intersection(&self, graph: &Self) -> Self {
        let mut graph_r: Graph<N, E, Directed> = self.clone();

        let self_indeces: HashSet<_> = self.node_identifiers().map(|n| self.to_index(n)).collect();
        let graph_indeces: HashSet<_> = graph
            .node_identifiers()
            .map(|n| graph.to_index(n))
            .collect();
        let remove_indeces = &self_indeces - &graph_indeces;
        remove_indeces.iter().for_each(|&n| {
            graph_r.remove_node(NodeIndex::new(n));
        });

        let mut edges_count: HashMap<(NodeIndex, NodeIndex), usize> = HashMap::new();
        for e in graph.edge_indices() {
            if let Some((n, m)) = graph.edge_endpoints(e) {
                *edges_count.entry((n, m)).or_insert(0) += 1;
            }
        }


        for e in graph_r.edge_indices().rev() {
            if let Some((n, m)) = graph_r.edge_endpoints(e) {
                if let Some(c) = edges_count.get(&(n, m)) {
                    println!("{:?}", (n, m , c));
                    if c == &0 {
                        graph_r.remove_edge(e);
                    } else {
                        *edges_count.entry((n, m)).or_insert(0) -= 1;
                    }
                } else {
                    graph_r.remove_edge(e);
                }
            }
        }

        return graph_r;
    }
}
