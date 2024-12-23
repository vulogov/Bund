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

With the help of the _save_ function, users can store the current state of the BUND VM into
an external file. This file, called the "WORLD" file, contains a frozen version of the scripts that could be executed during
bootstrap phase or by calling a _bootstrap_ function.
The _bootstrap_ function, which can be used as ```rust "WORLD_file" bootstrap```, will load content from this file and execute stored bootstrap scripts.
