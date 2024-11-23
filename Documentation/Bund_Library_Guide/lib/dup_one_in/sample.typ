#import "@preview/codelst:2.0.2": sourcecode

#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // Duplicating single value from stack "A"
  //
  @A 42
  @main
    :A dup_one_in
  // Now in stack A we have 42, 42 
  ```
]
