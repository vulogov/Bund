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

The _fold_stack_ function is designed to extract either all values stored in the named stack or just the values that precede the _nodata_ entry again, in the named stack. These extracted values will be compiled into a list and added back to the named stack.
