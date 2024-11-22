#import "@preview/chic-hdr:0.4.0": *
#include "settings.typ"
#import "globals.typ": *

#set page(
  paper: "iso-b5",
  margin: (bottom: 1.75cm, top: 2.25cm),
  fill: white
)
#set text(
  size: 13pt,
  font: "UniversalSerif.ttf",
  fill: black,
  weight: "regular",
  lang: "en",
  hyphenate: auto
)
#set par(justify: true)

#show: chic.with(
  chic-header(

  ),
  chic-footer(
    left-side: chic-page-number(),
  ),
  chic-separator(1pt),
  chic-offset(7pt),
  chic-height(1.5cm)
)

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
#set text(
  size: 13pt,
  font: "UniversalSerif.ttf",
  fill: black,
  weight: "regular",
  lang: "en",
  hyphenate: auto
)

#include "Finish.typ"
#include "License.typ"
#include "Content.typ"
