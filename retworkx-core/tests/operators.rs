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


#[cfg(test)]
mod test_intersection {
    use retworkx_core::operators::Intersection;
    use petgraph::Undirected;
    use petgraph::graph::Graph;

    use petgraph::visit::IntoNodeIdentifiers;

    #[test]
    fn test_empty_graph() {
        let g: Graph<(), (), Undirected> = Graph::new_undirected();
        let h: Graph<(), (), Undirected> = Graph::new_undirected();
        let r = g.intersection(&h);

        assert_eq!(r.node_count(), 0);
        assert_eq!(r.edge_count(), 0);
    }

    #[test]
    fn test_empty_digraph() {
        let g: Graph<(), ()> = Graph::new();
        let h: Graph<(), ()> = Graph::new();
        let r = g.intersection(&h);

        assert_eq!(r.node_count(), 0);
        assert_eq!(r.edge_count(), 0);
    }

    #[test]
    fn test_simple_graph() {
        let mut g: Graph<(), (), Undirected> = Graph::new_undirected();
        let gn1 = g.add_node(());
        let gn2 = g.add_node(());
        let gn3 = g.add_node(());
        let gn4 = g.add_node(());
        g.add_edge(gn1, gn2, ());
        g.add_edge(gn2, gn3, ());
        g.add_edge(gn3, gn4, ());

        let mut h: Graph<(), (), Undirected> = Graph::new_undirected();
        let hn1 = h.add_node(());
        let hn2 = h.add_node(());
        let hn3 = h.add_node(());
        h.add_edge(hn1, hn3, ());
        h.add_edge(hn3, hn2, ());
        h.add_edge(hn2, hn1, ());

        let r = g.intersection(&h);

        assert_eq!(r.node_count(), 3);
        assert_eq!(r.edge_count(), 2);
    }

    #[test]
    fn test_simple_digraph() {
        let g = Graph::<(), ()>::from_edges(&[(0, 1), (1, 2), (2, 3)]);
        let h = Graph::<(), ()>::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        let r = g.intersection(&h);

        assert_eq!(r.node_count(), 3);
        assert_eq!(r.edge_count(), 2);
    }

    #[test]
    fn test_multi_graph() {
        let mut g: Graph<(), (), Undirected> = Graph::new_undirected();
        let gn1 = g.add_node(());
        let gn2 = g.add_node(());
        let gn3 = g.add_node(());
        let gn4 = g.add_node(());
        g.add_edge(gn1, gn2, ());
        g.add_edge(gn2, gn3, ());
        g.add_edge(gn3, gn4, ());

        let mut h: Graph<(), (), Undirected> = Graph::new_undirected();
        let hn1 = h.add_node(());
        let hn2 = h.add_node(());
        let hn3 = h.add_node(());

        h.add_edge(hn1, hn3, ());
        h.add_edge(hn3, hn2, ());
        h.add_edge(hn2, hn1, ());

        let r = g.intersection(&h);

        assert_eq!(r.node_count(), 3);
        assert_eq!(r.edge_count(), 2);
    }

    #[test]
    fn test_multi_digraph() {
        let g = Graph::<(), ()>::from_edges(&[(0, 1), (0, 1), (0, 1), (1, 1), (1, 1)]);
        let h = Graph::<(), ()>::from_edges(&[(0, 1), (0, 1), (1, 1)]);
        let r = g.intersection(&h);
        for n in r.node_identifiers() {
            println!("{:?} {:?}", n, r[n])
        }
        for e in r.edge_indices() {
            let (n, m) = r.edge_endpoints(e).unwrap();
            println!("{:?} {:?} {:?}", e, n, m);

        }
        assert_eq!(r.node_count(), 2);
        assert_eq!(r.edge_count(), 3);
    }
}


