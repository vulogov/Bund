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

Create list filled with sequence of numbers according to configuration stored in the stack. Currently suppoered types of the sequence is:
- [ ] seq.ascending and this is the default. Parameters are: X, Step, N
- [ ] seq.descending. Parameters are: X, Step, N
