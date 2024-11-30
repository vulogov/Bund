#import "@preview/codelst:2.0.2": sourcecode


#sourcecode(
  numbers-side: right,
  gutter: 2em,
)[```rust
  //
  //
  //
  @A
    42
  @B
  @main
    42 :B :A move_from
  @B
    42 == {
            "Data moving from @A to @B happens"
            println } if
  ```
]
