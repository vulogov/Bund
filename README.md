# BUND concatenative language interpreter and shell.

Now, I am thrilled to introduce a novel concatenative programming language called BUND. What sets a concatenative language apart, and how does it diverge from the programming languages you are accustomed to? You are likely well-versed in applicative programming languages like Python, C, or Java. Alternatively, you may have dabbled in functional programming languages such as Lisp, Haskell, or ML, other instances of applicative programming languages. This category is characterized by the way functions are perceived and managed. In applicative languages, a function is treated as a mathematical primitive that computes based on passed arguments and returns a value.

On the other hand, concatenative programming languages facilitate the transfer of a data context from one function to another, external to the function itself. While the stack is the most common method for this transfer, there are concatenative languages that don't rely on a stack. This data context transfer enables the concatenation of data processing, making concatenative languages a practical choice for certain applications. While less prevalent in the software development communities, you might have come across languages such as Forth, PostScript, and Factor.

## Show me the code!

```rust
//
// This is faimous HelloWorld program
//
"Hello world!" println
```

## What is my options?

```shell
./target/debug/bund
Interpreter and CLI for a Virtual Machine for BUND programming language

Usage: bund [OPTIONS] <COMMAND>

Commands:
  script   Execute BUND script
  eval     Evaluate the BUND code snippet
  shell    Run the BUND REPL shell
  version  Get the version of the BUND
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...     Turn debugging information on
      --debug-shell  Drop to DEBUG shell if error occurs
      --nocolor      Disable colors in output
      --noeval       Disable bund.eval group of functions
      --noio         Disable I/O group of functions
  -h, --help         Print help
  -V, --version      Print version
```

### How to run a script ?

```bash
cat ./examples/helloworld.bund| ./target/debug/bund script --stdin
Hello World!
```

You can specify your script that you are intending for execution using following paramters for the _script_ subcommand

| CLI option | Description |
| --- | --- |
| --stdin | Execute script passed to STDIN |
| --file | Execute script stored in the file. *IMPORTANT*, full path must be provided. |
| --url | Execute script stored on some HTTP/HTTPS server |
| --eval| Execute code snippet from CLI |

### How to run interactive shell ?

You must execute _bund_ command with _shell_ subcommand

```shell
./target/debug/bund shell
  ____    _   _   _   _   ____       ___        _        ___
 | __ )  | | | | | \ | | |  _ \     / _ \      / |      / _ \
 |  _ \  | | | | |  \| | | | | |   | | | |     | |     | | | |
 | |_) | | |_| | | |\  | | |_| |   | |_| |  _  | |  _  | |_| |
 |____/   \___/  |_| \_| |____/     \___/  (_) |_| (_)  \___/


╭────────────────┬──────────────────────────────────────────────────────────╮
│ Hostname       ┆ XXXXXXXXXXXX                                             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ OS version     ┆ XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Kernel version ┆ XXXXXX                                                   │
╰────────────────┴──────────────────────────────────────────────────────────╯
[BUND >
```

### How to drop to debug shell if error occurs ?

You must pass _--debug-shell_ CLI parameter to _bund_ command

```shell
./target/debug/bund --debug-shell shell
  ____    _   _   _   _   ____       ___        _        ___
 | __ )  | | | | | \ | | |  _ \     / _ \      / |      / _ \
 |  _ \  | | | | |  \| | | | | |   | | | |     | |     | | | |
 | |_) | | |_| | | |\  | | |_| |   | |_| |  _  | |  _  | |_| |
 |____/   \___/  |_| \_| |____/     \___/  (_) |_| (_)  \___/

[BUND > 2 2 **
Attempt to evaluate value Value { id: "9fZEoerSihRUi_g1_Ic7K", stamp: 1729371784779.0, dt: 6, q: 100.0, data: String("**"), attr: [], curr: -1, tags: {} } returned error: i(**) for stack returned: Inline ** not registered 
[DEBUG >
```
