

#[derive(Clone)]
#[derive(Debug)]
struct Node (String);

impl Node {
    fn new(label: &str) -> Node {
        Node(label.to_string())
    }

    fn new_list(labels: Vec<&str>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        for label in labels {
            nodes.push(Node::new(label));
        }

        nodes
    }

}

#[derive(Clone)]
#[derive(Debug)]
struct Edge (Node, Node, i32);

impl Edge {
    fn new(from: Node, to: Node, weight: i32) -> Edge {
        Edge(from, to, weight)
    }

    fn new_list(args: Vec<(&Node, &Node, i32)>) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();

        for (from, to, weight) in args {
            let from: Node = from.clone();
            let to = to.clone();
            let weight = weight.clone();

            edges.push(Edge::new(from, to, weight));
        }

        edges

    }

}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn main() {

    let nodes: Vec<Node> = Node::new_list(Vec::from(["s", "v1", "v2", "v3", "v4", "v5", "v6", "t"]));

    let var: Vec<(&Node, &Node, i32)> = Vec::from([
        (&nodes[0], &nodes[1], 7),
        (&nodes[0], &nodes[2], 1),
        (&nodes[1], &nodes[3], 1),
        (&nodes[1], &nodes[3], 1),
        (&nodes[2], &nodes[3], 1),
        (&nodes[2], &nodes[4], 1),
        (&nodes[3], &nodes[5], 1),
        (&nodes[4], &nodes[5], 1),
        (&nodes[4], &nodes[6], 1),
        (&nodes[5], &nodes[7], 1),
        (&nodes[6], &nodes[7], 1),
    ]);

    let edges: Vec<Edge> = Edge::new_list(var);

    let graph: Graph = Graph {
        nodes: nodes,
        edges: edges,
    };
    // println!("{:?}", graph.nodes);
    // println!("{:?}", graph.edges);

    let kruskal_graph = kruskal_algorithm(&graph);


    // let prim_graph = prim_algorithm(&graph);

    println!("Hello, world!");
}

fn kruskal_algorithm(graph: &Graph) -> &Graph{

    let tree = Graph {
        nodes: vec![],
        edges: vec![],
    };

    tree.add_node(graph.nodes[0]);

    // verify graph



    graph
}

// fn prim_algorithm(graph: &Graph) -> &Graph{

//     // verify graph

//     graph
// }


// fn dijkstra(graph: &Graph, start: &Node, end: &Node) -> i32 {
//     let mut dist: Vec<i32> = vec![];
//     let mut prev: Vec<Node> = vec![];

//     for node in graph.nodes {
//         dist.push(i32::MAX);
//         prev.push(node);
//     }

//     dist[start] = 0;

//     let mut queue: Vec<Node> = graph.nodes.clone();

//     while queue.len() > 0 {
//         let mut u: Node = queue[0].clone();
//         let mut u_index: usize = 0;

//         for (i, node) in queue.iter().enumerate() {
//             if dist[node] < dist[u] {
//                 u = node.clone();
//                 u_index = i;
//             }
//         }

//         queue.remove(u_index);

//         for edge in graph.edges {
//             if edge.0 == u {
//                 let v = edge.1;
//                 let alt = dist[u] + edge.2;

//                 if alt < dist[v] {
//                     dist[v] = alt;
//                     prev[v] = u;
//                 }
//             }
//         }
//     }

//     dist[end]
// }
