
#import "globals.typ": *
#import "@preview/codelst:2.0.1": sourcecode
#import "@preview/treet:0.1.1": *

= BUND Standard library reference 
\
#include "Library_introduction.typ"

#let data = csv("index.csv", row-type: array)

#for row in data [
#pagebreak()
== #row.at(1)
#let index_file = str(("lib", row.at(0), "index.typ").join("/"))
#include str(index_file)
]
