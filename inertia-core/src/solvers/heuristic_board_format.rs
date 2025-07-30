use core::fmt;

use crate::solvers::HeuristicValue;

pub fn format_square_heuristics(
  f: &mut fmt::Formatter<'_>,
  squares: &[HeuristicValue; 256],
) -> fmt::Result {
  f.write_str("[\n")?;
  for row in squares.chunks(16) {
    f.write_str("  ")?;
    for (i, &square) in row.iter().enumerate() {
      if i > 0 {
        f.write_str(" ")?;
      }
      write!(f, "{:03}", square)?;
    }
    f.write_str("\n")?;
  }
  f.write_str("]")?;
  Ok(())
}
