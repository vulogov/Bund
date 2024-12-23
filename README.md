![Bund shell](screen.png)

# BUND concatenative language interpreter and shell.

I am thrilled to introduce a novel concatenative programming language called BUND. What sets a concatenative language apart, and how does it differ from the programming languages you are accustomed to?

You are likely well-versed in applicative programming languages like Python, C, or Java. Alternatively, you may have dabbled in functional programming languages such as Lisp, Haskell, or ML, which are also instances of applicative programming languages. This category is characterized by the way functions are perceived and managed. In applicative languages, a function is treated as a mathematical primitive that computes based on passed arguments and returns a value.

In contrast, concatenative programming languages facilitate the transfer of a data context from one function to another, external to the function itself. While the stack is the most common method for this transfer, some concatenative languages do not rely on a stack. This data context transfer enables the concatenation of data processing, making concatenative languages a practical choice for certain applications. Although less prevalent in software development communities, you might have come across languages such as Forth, PostScript, and Factor.

## Why shall I learn a new language ?

First and foremost, there is no "silver bullet" in the world of programming languages. Each language, that is delevoped and proposed may efficiently fit for one or more specific niches, aiding in software development. So, why concatenative languages and why BUND?

Concatenative languages offer several compelling benefits, particularly in terms of simplicity and efficiency. By allowing functions to be composed through concatenation, these languages enable a seamless flow of data, reducing the complexity associated with managing function inputs and outputs. This approach often results in more readable and maintainable code, as the sequence of operations is straightforward and intuitive. Additionally, the stack-based execution model commonly used in concatenative languages minimizes overhead and enhances performance, making them well-suited for resource-constrained environments. The minimalistic syntax further simplifies learning and usage, allowing developers to focus on the logic and functionality of their programs. Overall, concatenative languages provide a powerful and elegant framework for building efficient and maintainable software.

However, classic concatenative languages do have some faults. They are generally not very good at data storage and execution context separation and isolation. BUND addresses this problem by introducing named and anonymous data storage and execution contexts. Next, I shall speak about metaprogramming. Metaprogramming is a programming technique where programs have the ability to treat other programs as their data. This means that a program can be designed to read, generate, analyze, or transform other programs, and even modify itself while running. This approach allows for greater flexibility and abstraction, enabling developers to write more generic and reusable code. For many concatenative languages, metaprogramming is a feature that either does not exist or is "planned for the future." BUND provides advanced, out-of-the-box metaprogramming capabilities.

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
      --debugger     Run BUND code inside debugger
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

### How to drop to the BUND debugger

You have two options on how you can run the code snippet inside BUND debugger

#### Running entire script inside debugger

For that, you have to pass ```--debugger``` CLI parameter

#### Running code snippet inside BUND debugger interactively

You can interactively pass code snippet to a debug function

```shell
[BUND > "2 2 +" debug
╭────────────┬────────────────────────────────────────────────────────────╮
│ Value type ┆ Integer                                                    │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Value      ┆ 2                                                          │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Debug      ┆ Value { id: "vux_tEZu2Cxkk64itdxiz", stamp:                │
│            ┆ 1729884622410.0, dt: 2, q: 100.0, data: I64(2), attr: [],  │
│            ┆ curr: -1, tags: {} }                                       │
╰────────────┴────────────────────────────────────────────────────────────╯
[DEBUG >
╭────────────┬────────────────────────────────────────────────────────────╮
│ Value type ┆ Integer                                                    │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Value      ┆ 2                                                          │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Debug      ┆ Value { id: "QVqF2tbsIhMto4FiUx7e6", stamp:                │
│            ┆ 1729884622410.0, dt: 2, q: 100.0, data: I64(2), attr: [],  │
│            ┆ curr: -1, tags: {} }                                       │
╰────────────┴────────────────────────────────────────────────────────────╯
[DEBUG >
╭────────────┬────────────────────────────────────────────────────────────╮
│ Value type ┆ Call                                                       │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Value      ┆ Value(0)=Value { id: "QhjmkgKGPc6l9T9mbM5g0", stamp:       │
│            ┆ 1729884625035.0, dt: 0, q: 0.0, data: Null, attr: [],      │
│            ┆ curr: -1, tags: {} }                                       │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Debug      ┆ Value { id: "0hmYwhHhad8VMklPckEt3", stamp:                │
│            ┆ 1729884622410.0, dt: 6, q: 100.0, data: String("+"), attr: │
│            ┆ [], curr: -1, tags: {} }                                   │
╰────────────┴────────────────────────────────────────────────────────────╯
[DEBUG >
4
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
