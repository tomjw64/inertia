use fdg::{
  Force, ForceGraph, fruchterman_reingold::FruchtermanReingold, simple::Center,
};
use petgraph::stable_graph::StableGraph;

fn generate_graph() -> StableGraph<(), ()> {
  let mut g = StableGraph::new();

  let a = g.add_node(());
  let b = g.add_node(());
  let c = g.add_node(());

  g.add_edge(a, b, ());
  g.add_edge(b, c, ());
  g.add_edge(c, a, ());

  g
}

fn main() {
  let dataset = generate_graph();

  // Initialize a ForceGraph in 3 dimentions with random node positions from -10.0..=10.0.
  let mut graph: ForceGraph<f32, 3, (), ()> =
    fdg::init_force_graph_uniform(dataset, 10.0);

  // Apply the Fruchterman-Reingold (1991) force-directed drawing algorithm 100 times.
  FruchtermanReingold::default().apply_many(&mut graph, 100);
  // Center the graph's average around (0,0).
  Center::default().apply(&mut graph);

  // Render nodes:
  println!("nodes:");
  for (_, pos) in graph.node_weights() {
    println!("{pos:?}");
  }

  // Render edges:
  println!("edges:");
  for edge_idx in graph.edge_indices() {
    let (source_idx, target_idx) = graph.edge_endpoints(edge_idx).unwrap();

    println!(
      "{edge_idx:?}: {:?} to {:?}",
      &graph[source_idx].1, &graph[target_idx].1
    );
  }
}
