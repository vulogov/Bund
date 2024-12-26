#import "@preview/gentle-clues:1.0.0": *
\
#danger[
External file will be overwritten
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
    rows: 1,
    gutter: 5pt,
    fill: silver,
    grid.cell(
      align: left,
      fill: rgb("#DFE8CC"),
      include "algorithm.typ",
    ),
  )
  #include "sample.typ"
]
