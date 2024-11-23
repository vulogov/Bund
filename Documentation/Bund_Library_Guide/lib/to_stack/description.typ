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

Taking name of the stack from the stack and makes this stack a current stack. If stack doesn't exists VM creates it.
