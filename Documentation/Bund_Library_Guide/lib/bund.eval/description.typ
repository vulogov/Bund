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

This function is tasked with the evaluation of a code snippet that is retrieved from a string stored in the stack. The process involves standard error-handling mechanisms to manage any issues that may arise during execution. It is crucial to highlight that this function does not instantiate a new virtual machine."
