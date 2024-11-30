#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  // Creating message in TEXTBUFFER
  "Hello" .
    convert.to_textbuffer.
    "World" +.
  take println
  // And printing Hello World!
  ```
]
