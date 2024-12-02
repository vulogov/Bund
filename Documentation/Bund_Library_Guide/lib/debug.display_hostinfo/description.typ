#import "@preview/colorful-boxes:1.3.1": *
#import "@preview/cheq:0.2.2": checklist

#show: checklist

#colorbox(
  title: "Defined in",
  color: "default",
  radius: 2pt,
  width: auto
)[
  - [ ] #"rust_multistack"
  - [ ] #"rust_multistackvm"
  - [x] #"bund runtime"
]

Display information aout current host on which you are executing BUND code
