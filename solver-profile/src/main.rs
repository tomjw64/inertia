use clap::command;
use clap::Parser;
use colored::Colorize;
use inertia_core::mechanics::B64EncodedCompressedPosition;
use inertia_core::mechanics::CompressedPosition;
use inertia_core::mechanics::Position;
use inertia_core::solvers::astar;
use std::io;
use std::io::Write;
use std::time::Instant;

fn solve_and_time_named_position(
  name: &str,
  position_b64: String,
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

fn solve_encoded_position(position_b64: String) -> Option<usize> {
  let position = Position::try_from(
    CompressedPosition::try_from(B64EncodedCompressedPosition(position_b64))
      .unwrap(),
  )
  .unwrap();
  solve_position(&position)
}

fn solve_position(position: &Position) -> Option<usize> {
  astar::solve_position(position, 255).map(|v| v.0.len())
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
    let &(_, position_b64, expected_moves) =
      inertia_fixtures::get_sample_position(&name)
        .expect(&format!("Position with name '{}' does not exist!", name));
    solve_and_time_named_position(
      &name,
      position_b64.to_owned(),
      expected_moves,
    );
  } else {
    for &(name, position_b64, expected_moves) in
      inertia_fixtures::SAMPLE_POSITIONS
    {
      solve_and_time_named_position(
        name,
        position_b64.to_owned(),
        expected_moves,
      );
    }
  }
}
