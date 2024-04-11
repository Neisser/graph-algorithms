# Minimum Spanning Tree Algorithms in Rust

**This project implements two algorithms for finding the minimum spanning tree (MST) of a weighted, undirected graph: Kruskal's algorithm and Prim's algorithm. Both algorithms are provided in Rust and can handle adjacency matrices of any size containing weights.**

## Installation

This project requires Rust installed on your system. You can download and install it from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Usage

The project provides code for the algorithms but doesn't have a built-in executable. Here's how to use it:

1. **Clone the repository:** (assuming you have git installed)

```bash
git clone https://your-github-url/minimum-spanning-tree-rust.git
```

2. **Change directory:**

```bash
cd minimum-spanning-tree-rust
```

3. **Run the code:**

You can open the source code in any Rust IDE or text editor. The main functionalities are defined in the `Graph.rs` and `algorithms.rs` files. To test the algorithms, you can modify the sample adjacency matrix defined in `main.rs`.

For example, to find the MST using Prim's algorithm for the sample matrix:

![Alt text](img.png)

```rust
// In main.rs

let weights = vec![
  vec![0, 4, 0, 0, 0, 0, 0, 8],
  vec![4, 0, 8, 0, 0, 0, 0, 11],
  vec![0, 8, 0, 7, 0, 4, 0, 2],
  vec![0, 0, 7, 0, 9, 14, 0, 10],
  vec![0, 0, 0, 9, 0, 10, 2, 0],
  vec![0, 0, 4, 14, 10, 0, 3, 6],
  vec![0, 0, 0, 0, 2, 3, 0, 1],
  vec![8, 11, 2, 10, 0, 6, 1, 0],
];

let mst = prim(weights);

println!("Minimum Spanning Tree using Prim's Algorithm:");
for edge in mst {
  println!("({},{}) - weight: {}", edge.0, edge.1, weights[edge.0][edge.1]);
}

```

This code snippet defines a sample adjacency matrix and calls the `prim` function to find the MST. The output will display the edges and their corresponding weights in the MST.

Graph

```text

s0, v2 -> 1
v2, v6 -> 1
v1, v4 -> 2
v2, v4 -> 3
v1, v5 -> 4
v2, v5 -> 4
v3, v5 -> 5
v3, v6 -> 5
v4, t7 -> 6
v5, t7 -> 6
v6, t7 -> 6
s0, v1 -> 7
s0, v3 -> 9
```

Kruscal solution

```text
s,  v2 -> 1
v2, v6 -> 1
v1, v4 -> 2
v2, v4 -> 3
v1, v5 -> 4
v3, v5 -> 5
v4, t  -> 6
total -> 22
```

### Description of Algorithms

* **Kruskal's Algorithm:** This algorithm works by sorting the edges of the graph in non-decreasing order of their weights. It then iterates through the sorted edges and adds them to the MST if they don't create a cycle. The process continues until all vertices are connected in the MST.

* **Prim's Algorithm:** This algorithm starts with an arbitrary vertex and iteratively adds the edge with the minimum weight that connects the current MST to a non-included vertex. This process continues until all vertices are included in the MST.

### Project Structure

The project consists of the following files:

* `Graph.rs`: Defines the graph data structure and basic operations like adding edges.
* `algorithms.rs`: Implements Kruskal's and Prim's algorithms for finding the MST.
* `main.rs`: Provides an example of how to use the algorithms with a sample adjacency matrix.

### License

This project is licensed under the MIT License (see LICENSE file).
