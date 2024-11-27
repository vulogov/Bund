#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // Remove all data from named stack
  //
  @main
  @StackName
    1 2 3
  @main
    :StackName clear_in
  // After calling clear_in, stack with name "StackName"
  // will have no data
  ```
]
