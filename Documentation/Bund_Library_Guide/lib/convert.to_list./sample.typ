#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  pair :
    41 42
  ; . convert.to_list.
  [ 41 42 ] ==. {
    "Conversion is succesful"
    println
  } if
  ```
]
