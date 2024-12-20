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

Evaluating code snippet from the string that is loaded from file. Name of the file is taken from the string stored in the workbench. All errors are handled in a usual way. This function will not create a new virtual machine.
