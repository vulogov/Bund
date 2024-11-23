#import "@preview/gentle-clues:1.0.0": *
\
#danger[
This is destructive operation with stack.
]
\
#rect(
  width: 100%,
  fill: rgb("#E8D9CC"),
)[
  #include "description.typ"
]
\
#align(center)[
  #grid(
    columns: (100% - 20pt),
    rows: (auto,),
    gutter: 5pt,
    fill: silver,
    grid.cell(
      align: left,
      fill: rgb("#DFE8CC"),
      include "algorithm.typ",
    ),
    grid.cell(
      align: center,
      include "sample.typ",
    ),
  )
]
