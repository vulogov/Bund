#import "@preview/droplet:0.2.0": dropcap
#import "@preview/wrap-it:0.1.0": wrap-content

#let outlinename = "Content"
#let tpwidth = 160mm
#let dedication = "I want to thank my first teacher, who imparted the knowledge and guidance necessary to develop my first programs for the PDP-11 computer."
#let publishing-info = [
  "Строка 1",
  "Строка 2",
  "Строка 3"
]

#let make_chapter(ch_path) = {
  wrap-content(image(ch_path+"/header.jpeg", width: 38mm), text(style: "italic")[
    #include ch_path+"/header.typ"
  ])
  for (fname,) in csv(ch_path+"/index.csv") {
    dropcap(
      height: 3,
      justify: true,
      gap: 4pt,
      hanging-indent: 1em,
      overhang: 8pt,
      font: "ThornValley.ttf",
    )[
      #include ch_path+"/"+fname
    ]
  }
}

#let make_part(p_path) = {
  quote(block: true, quotes: true)[
    #include p_path+"/quote.txt"
  ]
  for (name, ch_name) in csv(p_path+"/index.csv") {
    [== #name ]
    make_chapter(ch_name)
  }
}

#let item(word) = {
  [#strong(word) #label("item") #box(width:0.2em)]
}

#let pos(word) = {(text(style: "italic")[#word])}
