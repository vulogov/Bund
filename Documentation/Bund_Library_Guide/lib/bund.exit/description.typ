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

Terminate BUND interpreter, exit and return an exit code to OS

#colorbox(
  title: "Aliases",
  color: "blue",
  radius: 2pt,
  width: auto
)[
  - [ ] #"exit"
]
