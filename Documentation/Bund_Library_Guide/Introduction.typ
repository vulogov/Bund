= Introduction
\
I will introduce a new concatenative programming language called BUND in this work. What is a concatenative language, and how does it differ from the programming languages you're likely familiar with? You're likely acquainted with applicative programming languages like Python, C, or Java. Alternatively, you may have discovered functional programming languages such as Lisp, Haskell, or ML, other examples of applicative programming languages. This category is defined by the way functions are viewed and handled. In applicative languages, a function is treated as a mathematical primitive that computes based on passed arguments and returns a value.
In contrast, concatenative programming languages pass a data context from one function to another, external to the function itself. While the stack is the most common method for passing such context, there are concatenative languages that don't utilize a stack. Passing data context enables the concatenation of data processing. Concatenative languages are less known in the software development communities, but you might have heard of languages such as Forth, PostScript, and Factor.
\
\
The stack is utilized in many but not all concatenative languages, while applicative languages often use stack structures internally to aid computation. Stacks are indispensable for recursive computation, passing return values computed by functions and storing references to an execution context. What distinguishes concatenative stack-based languages from applicative counterparts is the use of the stack for input data, computational context, and result storage. In essence, everything in concatenative stack-based languages is stored in the stack. In some cases, computational instructions are also stored alongside data on the stack. Since everything, including the context for functions, is stored on the stack, functions in concatenative stack-based languages do not have conventional arguments. Although they function as such, they are often referred to as "words," as was defined in one of the first concatenative languages to gain popularity - Forth. Another characteristic of concatenative stack-based languages is their reliance on the stack's Last In, First Out (LIFO) nature. They often employ Reverse Polish Notation (RPN).
\
\
So, what will might surprise you in concatenative stack-based language?
- We already mentioned that the functions do not have arguments and no dedicated return value. All input and output data passed to and from the function are passed through the stack.
- You are responsible for ensuring the correct order of the values passed in the data context to the function, as this context is on the stack.
- You are also responsible for interpreting return data placed on the stack. Unlike in the functional language paradigm, there could be more than one return value, depending on your function (or "word").
- There are no variables. All data are stored on the stack.
- There are no global constants, variables, or values. Everything is on the stack.
- Due to the LIFO nature of the stack, you will deal with RPN, although BUND offers you an ability to creae a stack with FIFO policy.
