#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // Folding data in the stack with name "A"
  @main
  @A
    1 2 3 nodata 4 5 6
  @main
    :A fold_stack
  // Result of the executing of this
  // function will be 1 2 3 [4 5 6]
  // in the stack "A"
  ```
]
