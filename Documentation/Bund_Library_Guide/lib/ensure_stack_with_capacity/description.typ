#import "@preview/colorful-boxes:1.3.1": *
#import "@preview/cheq:0.2.2": checklist

#show: checklist

#colorbox(
  title: "Defined in",
  color: "default",
  radius: 2pt,
  width: auto
)[
  - [x] #"rust_multistack"
  - [ ] #"rust_multistackvm"
  - [ ] #"bund runtime"
]

Make named stack current, create if stack does not exists with defined capacity
