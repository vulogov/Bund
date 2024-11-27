#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // This snippet will check if stack with name "A"
  // exists and prints the message
  //
  @A
  :A
    stack_exists
      { "Stack A existing" } ?
  // And yes, it ddoes exists, as @A will make sure that it does
  :B
    stack_exists
    not { "There is no stack with name B" } ?
  // And stack B doesn't exists
  ```
]
