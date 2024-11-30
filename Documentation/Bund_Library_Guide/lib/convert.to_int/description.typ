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
  - [x] #"rust_multistackvm"
  - [ ] #"bund runtime"
]

Taking value from the stack, converting to the INT and pushing result to the stack
