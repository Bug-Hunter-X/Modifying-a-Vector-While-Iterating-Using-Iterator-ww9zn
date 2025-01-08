# Modifying a Vector While Iterating Using Iterator in Rust

This repository demonstrates a common error in Rust: modifying a vector while iterating over it using a mutable iterator. This can lead to unexpected behavior and crashes.

The `bug.rs` file contains the buggy code, which attempts to modify the vector while an iterator is traversing it. This action invalidates the iterator's state, making the subsequent iteration unpredictable.

The `bugSolution.rs` file provides a corrected version. It demonstrates safe ways to modify a vector during iteration, such as creating a new vector or using indexing.