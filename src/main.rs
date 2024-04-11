use std::collections::HashMap;
#[derive(Clone)]
struct MatrixEdges(Vec<Vec<u32>>);

#[derive(Clone)]
struct AdjacencyMatrix {
    edges: MatrixEdges,
}

impl AdjacencyMatrix {
    fn new(edges: MatrixEdges) -> AdjacencyMatrix {
        AdjacencyMatrix { edges: edges }
    }
}

struct Graph {
  num_vertices: usize,
  adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
  fn new(num_vertices: usize) -> Self {
    Graph {
      num_vertices,
      adjacency_list: HashMap::new(),
    }
  }

  fn add_edge(&mut self, u: usize, v: usize) {
    self.adjacency_list.entry(u).or_insert(Vec::new()).push(v);
    self.adjacency_list.entry(v).or_insert(Vec::new()).push(u);
  }

  fn is_ciclic_util(&self, vertex: usize, visited: &mut Vec<bool>, parent: usize) -> bool {
    visited[vertex] = true;

    let neighbors = self.adjacency_list.get(&vertex).unwrap_or(&Vec::new()).clone();

    for neighbor in &neighbors {
      if !visited[*neighbor] {
        if self.is_ciclic_util(*neighbor, visited, vertex) {
          return true;
        }
      } else if *neighbor != parent {
        return true;
      }
    }

    false
  }

  fn is_tree(&self) -> bool {
   let mut visited = vec![false; self.num_vertices];

    // check for a cycle starting from vertex 0
    if self.is_ciclic_util(0, &mut visited, usize::MAX){
      return false;
    }

    // check if all vertices are connected
    // for vertex in 0..self.num_vertices {
    //   if !visited[vertex] {
    //     return false;
    //   }
    // }

    true
  }
}

struct SpanningTree {
  num_vertices: usize,
  edges: Vec<Edge>,
  total_weight: u32,
}

impl SpanningTree {
  fn new(num_vertices: usize) -> SpanningTree {
      SpanningTree {
          num_vertices,
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

  fn is_tree(&self, new_edge: &Edge) -> bool {
    // Create a HashMap to store the parent of each node.
    let Edge ( new_from, new_to, _) = new_edge;

    let mut graph = Graph::new(self.num_vertices);

    graph.add_edge(*new_from as usize, *new_to as usize);

    // Perform DFS to check for cycles.
    for edge in self.edges.iter() {
        let Edge(from, to, _) = edge;
        graph.add_edge(*from as usize, *to as usize);
    }

    graph.is_tree()
}


}

#[derive(Debug)]
struct Edge(u32, u32, u32);

fn main() {
  println!("Hello, world!");

  let graph: AdjacencyMatrix = AdjacencyMatrix::new(MatrixEdges(vec![
    vec![0, 7, 1, 9, 0, 0, 0, 0],
    vec![7, 0, 0, 0, 2, 4, 0, 0],
    vec![1, 0, 0, 0, 3, 4, 1, 0],
    vec![9, 0, 0, 0, 0, 5, 5, 0],
    vec![0, 2, 3, 0, 0, 0, 0, 6],
    vec![0, 4, 4, 5, 0, 0, 0, 6],
    vec![0, 0, 1, 5, 0, 0, 0, 6],
    vec![0, 0, 0, 0, 6, 6, 6, 0],
]));

  let mst_kruskal: SpanningTree = kruskal(graph.clone());

  println!("Kruskal Algorithm");
  mst_kruskal.display();

  let mst_prim: SpanningTree = prim(graph);

  println!("Prim Algorithm");
  mst_prim.display();

}


fn kruskal(graph: AdjacencyMatrix) -> SpanningTree {
  let mut mst = SpanningTree::new(graph.edges.0.len());
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

  println!("{:?}", edges);

  for edge in &edges {
    let Edge(u, v, weight) = edge;
    let is_tree = mst.is_tree(edge);

    // validate if the edge creates a cycle
    if is_tree {

        mst.add_edge(*u, *v, *weight);

        mst.add_weight(*weight)
    }
  }


  mst
}


fn prim(graph: AdjacencyMatrix) -> SpanningTree {
  let n = graph.edges.0.len();
  let mut mst = SpanningTree::new(n);

  let mut visited = Vec::from(vec![false; n]);

  // choose an arbitrary vertex to start the tree
  let mut current_vertex = 0;
  visited[current_vertex] = true;

  // loop until all vertices are visited
  for _ in 0..n - 1 {
    let mut min_weight = u32::MAX;
    let mut next_vertex = 0;

    //find the next vertex to add to the tree
    for i in 0..n {
      if visited[i] {
        for j in 0..n {
          if !visited[j] && graph.edges.0[i][j] != 0 && graph.edges.0[i][j] < min_weight {
            min_weight = graph.edges.0[i][j];
            current_vertex = i;
            next_vertex = j;
          }
        }
      }
    }

    mst.add_edge(current_vertex as u32, next_vertex as u32, min_weight);
    mst.add_weight(min_weight);
    visited[next_vertex] = true;
  }

  mst
}