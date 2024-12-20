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


Users can pass arguments to the `script` and `shell` subcommands after `--`. For example, to pass arguments to the `shell` subcommand, you would use:

```bash
bund shell -- "Argument1" 2 3
```

The "word" `args` will return to the top of the stack list containing the passed arguments.
