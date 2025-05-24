use fdg::{
  fruchterman_reingold::FruchtermanReingold,
  nalgebra::{Point, Vector3},
  petgraph::stable_graph::StableGraph,
  simple::Center,
  Force,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::mechanics::{Direction, MoveBoard, Position, Square};

const TARGET_COLOR: &'static str = "#ff0000";
const SQUARE_COLOR: &'static str = "#00ff00";

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Graph {
  pub nodes: Vec<GraphNode>,
  pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GraphCoordinates {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GraphNode {
  pub color: String,
  pub position: GraphCoordinates,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GraphEdge {
  pub source: usize,
  pub target: usize,
}

#[wasm_bindgen]
pub fn generate_force_graph_from_position(
  position: Position,
  t: usize,
) -> Graph {
  let mut force_graph = StableGraph::<((), Point<f32, 3>), ()>::new();

  let move_board = MoveBoard::from(&position.walled_board);

  let mut fg_nodes = Vec::with_capacity(256);
  let mut fg_edges = Vec::with_capacity(256);

  (0..=255).for_each(|i: u8| {
    let (row, col) = Square(i).as_row_col();
    // Set initial position to manhattan distance from goal
    let (goal_row, goal_col) = position.goal.as_row_col();
    let node = force_graph.add_node((
      (),
      Point::from(Vector3::new(
        col as f32,
        -1.0
          * (usize::abs_diff(goal_row, row) as f32
            + usize::abs_diff(goal_col, col) as f32),
        row as f32,
      )),
    ));
    fg_nodes.push(node);
  });

  (0..=255).for_each(|i: u8| {
    for direction in Direction::VARIANTS {
      let move_destination =
        move_board.get_unimpeded_move_destination(Square(i), direction);
      let source = fg_nodes[i as usize];
      let target = fg_nodes[move_destination.0 as usize];
      if source == target {
        continue;
      }
      force_graph.add_edge(source, target, ());
      fg_edges.push(GraphEdge {
        source: i as usize,
        target: move_destination.0 as usize,
      });
    }
  });

  FruchtermanReingold::default().apply_many(&mut force_graph, t);
  Center::default().apply(&mut force_graph);

  Graph {
    nodes: fg_nodes
      .into_iter()
      .map(|node_index| force_graph[node_index])
      .enumerate()
      .map(|(i, (_, pos))| GraphNode {
        color: if Square(i as u8) == position.goal {
          TARGET_COLOR.to_string()
        } else {
          SQUARE_COLOR.to_string()
        },
        position: GraphCoordinates {
          x: pos.x,
          y: pos.y,
          z: pos.z,
        },
      })
      .collect(),
    edges: fg_edges,
  }
}
