#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  // Print number of lines in /etc/passwd
  //
  "There are " print
  "/etc/passwd" file . lines. take len print
  " lines in /etc/passwd" println
  ```
]
