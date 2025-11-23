use std::hash::BuildHasherDefault;
use std::hash::Hasher;

pub struct NoopHasher(u64);

impl Hasher for NoopHasher {
  fn finish(&self) -> u64 {
    self.0
  }

  fn write(&mut self, _bytes: &[u8]) {
    unimplemented!()
  }

  fn write_u64(&mut self, val: u64) {
    self.0 = val;
  }
}

impl Default for NoopHasher {
  fn default() -> Self {
    Self(0)
  }
}

pub type NoopHasherBuilder = BuildHasherDefault<NoopHasher>;
