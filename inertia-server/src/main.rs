use inertia_core::board_generators::ClassicBoardGenerator;
use inertia_core::board_generators::EmptyMiddleGoalBoardGenerator;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::WalledBoardPosition;
use inertia_core::mechanics::WalledBoardPositionGenerator;
use inertia_core::solvers::idas::deepening_search_to_depth;
use inertia_core::solvers::SolutionStep;
use rouille::router;

fn main() {
  let server_address = "0.0.0.0:8000";
  println!("Now listening on {}", server_address);

  rouille::start_server(server_address, move |request| -> rouille::Response {
    router!(request,
      (GET) (/board/random) => {
        let position = ClassicBoardGenerator::new().generate_position();
        let WalledBoardPosition { walled_board, actor_squares, goal } = position;
        let board = MoveBoard::from(&walled_board);

        let solution: Option<Vec<SolutionStep>> =
          deepening_search_to_depth(&board, goal, actor_squares, 45);
        println!("{:?}", solution);
        println!("{}", serde_json::to_string(&solution).unwrap());
        dbg!(solution.map(|v| v.len()));

        rouille::Response::json(&position).with_additional_header("Access-Control-Allow-Origin", "*")
      },
      _ => rouille::Response::empty_404()
    )
  });
}
