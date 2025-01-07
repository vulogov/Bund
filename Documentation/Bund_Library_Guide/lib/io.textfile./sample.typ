#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // Print /etc/passwd on console
  //
  "/etc/passwd" . io.textfile. take { println } loop
  ```
]
