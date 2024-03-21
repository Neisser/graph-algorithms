use std::collections::HashMap;

// Disjoint-Set Union (DSU) structure for efficient cycle detection
struct DisjointSetUnion {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]);
        }
        self.parent[u]
    }

    fn union(&mut self, u: usize, v: usize) {
        let u_root = self.find(u);
        let v_root = self.find(v);

        if u_root != v_root {
            if self.rank[u_root] < self.rank[v_root] {
                self.parent[u_root] = v_root;
            } else if self.rank[u_root] > self.rank[v_root] {
                self.parent[v_root] = u_root;
            } else {
                self.parent[u_root] = v_root;
                self.rank[v_root] += 1;
            }
        }
    }
}

struct MatrixEdges(Vec<Vec<u32>>);
struct AdjacencyMatrix {
    edges: MatrixEdges,
}

impl AdjacencyMatrix {
    fn new(edges: MatrixEdges) -> AdjacencyMatrix {
        AdjacencyMatrix { edges: edges }
    }
}

#[derive(Debug)]
struct Edge(u32, u32, u32);

struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Graph {
        Graph { edges: Vec::new() }
    }

    fn add_edge(&mut self, from: u32, to: u32, weight: u32) {
        self.edges.push(Edge(from, to, weight));
    }
}

struct SpanningTree {
    edges: Vec<Edge>,
    total_weight: u32,
}

impl SpanningTree {
    fn new() -> SpanningTree {
        SpanningTree {
            edges: Vec::new(),
            total_weight: 0,
        }
    }

    fn add_edge(&mut self, from: u32, to: u32, weight: u32) {
        self.edges.push(Edge(from, to, weight));
    }

    fn add_weight(&mut self, weight: u32) {
        self.total_weight += weight;
    }

    fn display(&self) {
        println!("display");
        println!(
            "Total weight for minimun spanning tree: {}",
            self.total_weight
        );
        self.edges.iter().for_each(|edge| {
            println!("{:?}", edge);
        });
    }

    fn contain_cicle(&self, new_edge: &Edge) -> bool {
        // Create a HashMap to store the parent of each node.
        let mut parent: HashMap<u32, u32> = HashMap::new();
        let Edge ( new_from, new_to, _) = new_edge;

        // Perform DFS to check for cycles.
        for edge in self.edges.iter() {
            let Edge ( curr_from, curr_to, _) = edge;

            if let Some(existing_parent) = parent.get(&curr_to) {
                // Cycle detected if a node already has a parent and it's not the same as the current edge's source.
                if existing_parent != curr_from {
                    return true;
                }
            } else {
                parent.insert(*curr_to, *curr_from);
            }
        }

        // Check for cycle with the new edge. Simulate adding the new edge and performing DFS.
        if let Some(existing_parent) = parent.get(&new_to) {
            if existing_parent != new_from {
                return true;
            }
        }

        // No cycle found so far. Return false.
        false

    }

    fn has_circle(&self, edge: &Edge) -> bool {
        let Edge(u, v, _) = *edge; // Dereference for cleaner access

        // Use Disjoint-Set Union (DSU) to efficiently track connected components
        let mut dsu = DisjointSetUnion::new(self.edges.len() + 1); // +1 for potential new edge

        // Check if the graph has at least 3 edges
        // if self.edges.len() < 3 {
        //     dsu.union(u as usize, v as usize);

        //     return false;
        // }

        // Check if adding the edge connects previously connected components
        for &Edge(from, to, _) in &self.edges {
            if dsu.find(from as usize) == dsu.find(to as usize) {
                return true; // Cycle detected if already connected
            }
            // dsu.union(from as usize, to as usize);
        }

        // No cycle detected if adding the edge doesn't connect existing components
        // dsu.union(u as usize, v as usize);
        false
    }
}

fn main() {
    let graph: AdjacencyMatrix = AdjacencyMatrix::new(MatrixEdges(vec![
        vec![0, 7, 1, 9, 0, 0, 0, 0],
        vec![7, 0, 0, 0, 2, 4, 0, 0],
        vec![1, 0, 0, 0, 3, 0, 1, 0],
        vec![9, 0, 0, 0, 0, 5, 5, 0],
        vec![0, 2, 3, 0, 0, 0, 0, 6],
        vec![0, 4, 0, 5, 0, 0, 0, 6],
        vec![0, 0, 1, 5, 0, 0, 0, 6],
        vec![0, 0, 0, 0, 6, 6, 6, 0],
    ]));

    let mst_kruskal: SpanningTree = kruskal(graph);

    mst_kruskal.display();

    // let mst: SpanningTree = prim(graph);
    // println!("Total weight for minimun spanning tree: {}", mst.total_weight);
    // println!("{:?}", mst.edges);

    println!("Hello, world!");
}

fn kruskal(graph: AdjacencyMatrix) -> SpanningTree {
    let mut mst = SpanningTree::new();
    let mut edges: Vec<Edge> = Vec::new();

    graph.edges.0.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, weight)| {
            if *weight != 0 && (i < j) {
                // Only add the edges once

                edges.push(Edge(i as u32, j as u32, *weight));
            }
        });
    });

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    for edge in &edges {
        let Edge(u, v, weight) = edge;
        println!(
            "u {:?} v {:?} w {:?} cicle {:?}",
            u,
            v,
            weight,
            mst.contain_cicle(&edge)
        );

        // validate if the edge creates a cycle
        if !mst.contain_cicle(&edge) {
            mst.add_edge(*u, *v, *weight);
            mst.add_weight(*weight)
        }
    }

    // edges.iter().for_each(|edge| {
    //     let Edge(u, v, weight) = edge;
    //     println!(
    //         "u {:?} v {:?} w {:?} cicle {:?}",
    //         u,
    //         v,
    //         weight,
    //         mst.has_circle(&edge)
    //     );

    //     // validate if the edge creates a cycle
    //     if !mst.has_circle(&edge) {
    //         mst.add_edge(*u, *v, *weight);
    //         mst.add_weight(*weight)
    //     }
    // });

    // edges.iter().for_each(|edge| {
    //     println!("{:?}", edge);
    // });

    mst
}

// fn prim(graph: Graph) -> SpanningTree {
//     let n: usize = graph.edges.len();
//     let mut mst: SpanningTree = SpanningTree::new();

//     let mut visited: Vec<bool> = Vec::from(vec![false; n]);

//     println!("{:?}", visited);

//     // Choose an arbitrary vertex as a starting point
//     let mut current_vertex: usize = 0;
//     visited[current_vertex] = true;

//     // Loop through all remaining vertices
//     for _ in 0..n - 1 {
//         let mut min_weight: u32 = std::u32::MAX;
//         let mut next_vertex: usize = 0;

//         // Find the unvisited vertex with the smallest edge weight
//         for i in 0..n {
//             // println!("i value up {:?}", i);
//             // println!("visited value up {:?}", visited[i]);
//             if !visited[i] {
//                 for j in 0..n {
//                     // println!("j value up {:?}", j);
//                     // println!("visited value {:?}", visited[j]);
//                     if visited[j] && graph.edges[j][i] < min_weight {
//                         min_weight = graph.edges[j][i];
//                         // println!("min weight {:?}", min_weight);
//                         // println!("i value {:?}", i);
//                         // println!("j value {:?}", j);
//                     }
//                 }
//             }
//         }

//             // Add the edge to the minimum spanning tree
//             mst.edges.push((current_vertex as u32, next_vertex as u32));
//             mst.total_weight += min_weight;

//             // Mark the vertex as visited
//             visited[next_vertex] = true;

//             // Update current vertex
//             current_vertex = next_vertex;
//     }

//     mst.edges.push((current_vertex as u32, 0)); // Agregar la última arista para completar el ciclo
//     mst.total_weight += graph.edges[current_vertex][0];

//     mst
//   }
