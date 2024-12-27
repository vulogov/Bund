#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // Generate descending sequence
  //
  config
    "type"  "seq.descending" set
    "X"     100.0       set
    "Step"  3.0         set
    "N"     3           set
  "Generated sequence is: " print seq println
  ```
]
