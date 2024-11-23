#import "@preview/colorful-boxes:1.3.1": *
#import "@preview/gentle-clues:1.0.0": *
#import "@preview/cheq:0.2.2": checklist

#show: checklist

Although language design is often simple, elegant, and thoughtfully executed, there is room for greater practicality. A language's core becomes truly functional and valuable to developers only when accompanied by a standard library of useful functions. These functions provide essential tools for performing operations and manipulating data effectively. Moreover, BUND shares several characteristics with other concatenative languages.
\
#memo[
All run-time functionality of the BUND implemented in standard library.
]
\
When I say "all," I mean that every aspect of the functionality extends beyond just implementing the BUND parser and core logic. The BUND standard library is situated across multiple locations. Although this may initially be a design flaw, I had deliberate reasons for structuring the standard library this way.
\
#info[
  - [ ] The Rust crate _rust_multistack_ encompasses all the logic associated with stack operations. Additionally, it incorporates elements of the standard library that pertain specifically to these operations. Features include data swapping on the stack, data duplication, removal of data, stack rotation, creation of stacks, and other related functionalities.
  - [ ] The Rust crate _rust_multistackvm_ is a foundational implementation of the BUND virtual machine. Although different tools and interpreters may facilitate access to BUND, the core logic of the BUND language remains intact within this crate. This encompasses data manipulation and conversion, application logic, mathematical operations, lambda function processing, and all other essential features.
  - [ ] The Bund runtime serves as the interpreter for the Bund programming language. It encompasses implementing all standard library functions and facilitating user-related features and controls.
]
\
