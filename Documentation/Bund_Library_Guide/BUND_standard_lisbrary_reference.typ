#include "settings.typ"
#import "globals.typ": *

#set page(
  paper: "iso-b5",
  margin: (bottom: 1.75cm, top: 2.25cm),
  fill: white
)
#set text(
  size: 13pt,
  font: "universalserif.ttf",
  fill: black,
  weight: "regular",
  lang: "en",
  hyphenate: auto
)
#set par(justify: true)

#show heading.where(level: 1): it => {
  pagebreak(to: "odd")
  v(5%)
  text(
    2em,
    weight: 400,
    block([#it.body])
  )
  v(1.25em)
}

#set quote(block: true)
#show quote: set align(right)

#include "Title.typ"
#include "Introduction.typ"
#include "How_to_use.typ"
#include "Library.typ"

#set text(
  size: 13pt,
  font: "universalserif.ttf",
  fill: black,
  weight: "regular",
  lang: "en",
  hyphenate: auto
)

#include "Finish.typ"
#include "License.typ"
#include "Content.typ"
