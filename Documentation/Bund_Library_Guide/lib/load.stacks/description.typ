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
an external file. This file, called the "WORLD" file, contains a frozen version of the
registered lambda functions, user functions, aliases, and the content of the stacks.
The _load.stacks_ function, which can be used as ```rust "WORLD_file" load.stacks```, will load stacks content from this file and add it to the existing data contexts.
