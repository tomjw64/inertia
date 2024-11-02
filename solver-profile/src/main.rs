use base64::engine::general_purpose;
use base64::Engine;
use clap::command;
use clap::Parser;
use colored::Colorize;
use inertia_core::mechanics::Position;
use inertia_core::solvers::astar;
use std::io;
use std::io::Write;
use std::time::Instant;

const POSITIONS: &[(&str, &str, usize)] = &[
  ("one-move", "EAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEAAQABAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAgME", 1),
  ("shuffle", "_38AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA_3__fwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD_fxESISKI", 70),
  ("empty", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABEBGI", 43),
  ("gen_15", "QBAIAQAAAABQAAAAADBAAUABAkBABAAAQAAAAAAgBBACAQACAAAAAAEAEAAIDEABQgEAAAACAAAgAAAgQAEQBCVsOTK4", 15),
  ("gauntlet", "AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFQEAEBHu", 73),
  ("gauntlet_guardrail", "CAAgACAAgQCBAAUCBQIVCBUIVSBVIFUBVQFVBVUFVRUIACAAIACBAIEABQIFAhUIFQhVIFUgVQFVAVUFVQVVFQEAEBHu", 36),
  ("gauntlet_close", "AAAAAAAAAQABAAUABQAVABUAVQBVAFUBVQFVBVUFVRUAAAAAAAABAAEABQAFABUAFQBVAFUAVQFVAVUFVQVVFd8A_RHu", 63),
  ("gauntlet_close_guardrail", "CAAgACAAgQCBAAUCBQIVCBUIVSBVIFUBVQFVBVUFVRUIACAAIACBAIEABQIFAhUIFQhVIFUgVQFVAVUFVQVVFd8A_RHu", 38)
];

// TODO: from random generation
// CBAAAgABQgAgAAAoEgJAAUAhKgAAJAARQAAAACAAkABACCACBAAAAAABQCIQCEgBQAUkAAIAIAIABJAAAAQCAZXQdaNd
// AQGAAAIAAAAAAAQECAhAAUAhJAAAEQACCAAABAAAiAACBAAAJABACgAAAAAAAkABQgEAChAAACBAAgABAAAQBO-uKnmT

fn solve_and_time_named_position(
  name: &str,
  position_b64: &str,
  expected_moves: usize,
) {
  print!("Solving {}: ", name);
  io::stdout().flush().expect("Unable to flush stdout");
  let start = Instant::now();

  let solution = solve_encoded_position(position_b64);

  let elapsed = start.elapsed();
  let result = if solution == Some(expected_moves) {
    "Succeeded".green()
  } else {
    "Failed".red()
  };
  println!(
    "[{}] in {:.2?} and {} move(s)",
    result,
    elapsed,
    solution.map(|v| v.to_string()).unwrap_or("-".to_owned())
  );
}

fn solve_encoded_position(position_b64: &str) -> Option<usize> {
  let bytes = general_purpose::URL_SAFE_NO_PAD
    .decode(position_b64)
    .unwrap();
  let position =
    Position::from_compressed_byte_array(&bytes[0..69].try_into().unwrap());
  solve_position(&position)
}

fn solve_position(position: &Position) -> Option<usize> {
  astar::solve_position(position, 255).map(|v| v.len())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(long)]
  name: Option<String>,
}

fn main() {
  let args = Args::parse();

  if let Some(name) = args.name {
    let &(__, position_b64, expected_moves) = POSITIONS
      .iter()
      .find(|item| item.0 == name)
      .expect(&format!("Position with name '{}' does not exist!", name));
    solve_and_time_named_position(&name, position_b64, expected_moves);
  } else {
    for &(name, position_b64, expected_moves) in POSITIONS {
      solve_and_time_named_position(name, position_b64, expected_moves);
    }
  }
}
