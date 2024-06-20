// Note: Because popped priority is only ever monotonic, this structure only
// works for some heuristics. If there exists any move (which costs 1) from an
// arrangement with heuristic N to another with heuristic N-2, this data
// structure will not work, because it would be able to create priorities lower
// than that of the currently searched bucket.
pub struct BucketingMonotonicPriorityQueue<T> {
  buckets: Vec<Vec<T>>,
  current_bucket: usize,
  capacity_per_bucket: usize,
}

impl<T> BucketingMonotonicPriorityQueue<T> {
  pub fn with_capacities(
    capacity: usize,
    capacity_per_bucket: usize,
  ) -> BucketingMonotonicPriorityQueue<T> {
    BucketingMonotonicPriorityQueue {
      buckets: Vec::with_capacity(capacity),
      current_bucket: 0,
      capacity_per_bucket,
    }
  }

  pub fn push(&mut self, value: T, priority: usize) {
    if self.buckets.len() <= priority {
      self.buckets.resize_with(priority + 1, || {
        Vec::with_capacity(self.capacity_per_bucket)
      });
    }
    self.buckets[priority].push(value);
  }

  pub fn pop(&mut self) -> Option<T> {
    while self.current_bucket < self.buckets.len() {
      let value = self.buckets[self.current_bucket].pop();
      if value.is_some() {
        return value;
      }
      // The current bucket is empty, and will never be used again. Let's shrink
      // it to hand memory back to the allocator.
      self.buckets[self.current_bucket].shrink_to_fit();
      self.current_bucket += 1;
    }
    None
  }
}

// If we need something not monotonic...
pub struct BucketingPriorityQueue<T> {
  buckets: Vec<Vec<T>>,
  current_bucket: usize,
  capacity_per_bucket: usize,
}

impl<T> BucketingPriorityQueue<T> {
  pub fn with_capacities(
    capacity: usize,
    capacity_per_bucket: usize,
  ) -> BucketingPriorityQueue<T> {
    BucketingPriorityQueue {
      buckets: Vec::with_capacity(capacity),
      current_bucket: 0,
      capacity_per_bucket,
    }
  }

  pub fn push(&mut self, value: T, priority: usize) {
    if self.buckets.len() <= priority {
      self.buckets.resize_with(priority + 1, || {
        Vec::with_capacity(self.capacity_per_bucket)
      });
    }
    if priority < self.current_bucket {
      self.current_bucket = priority;
    }
    self.buckets[priority].push(value);
  }

  pub fn pop(&mut self) -> Option<T> {
    while self.current_bucket < self.buckets.len() {
      let value = self.buckets[self.current_bucket].pop();
      if value.is_some() {
        return value;
      }
      self.buckets[self.current_bucket].shrink_to(self.capacity_per_bucket);
      self.current_bucket += 1;
    }
    None
  }
}
