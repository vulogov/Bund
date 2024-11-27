#import "@preview/fletcher:0.5.1" as fletcher: diagram, node, edge
#import fletcher.shapes: rect, ellipse

#pagebreak()

== What exactly is a Bund?
\
\
The BUND programming language is a member of the concatenative language family. A notable characteristic of concatenative languages is the presence of a computational context external to the code itself. All computations carried out by the functions, referred to as "words" in concatenative language terminology, are performed over this external context. This differs from the concepts commonly encountered in applicative languages, where function parameters are part of the function context. The computational context is typically structured as a Last In, First Out (LIFO) stack in concatenative languages. However, BUND distinguishes itself from most concatenative languages by having a more sophisticated concept of the computational context.
\
#pagebreak()
=== Circular data stack
\
\
Instead of using simple LIFO stacks, BUND stores data in multiple named circular buffers, also known as stacks. When you push data to the stack, the circular buffer expands, and when you pull or consume data from the stack, the buffer contracts. While the data buffer is circular, there is always a pointer that refers to the value located on top of the stack. Although you can rotate the buffer in the left or right direction, data is consumed in a single direction only.
\
\
\
\
\
#set text(9pt)
#align(center)[
  #diagram(
    node-stroke: .1em,
    node-fill: gradient.radial(blue.lighten(80%), blue, center: (30%, 20%), radius: 80%),
    spacing: 2em,
    node((0,0), "Value [0]", radius: 2em, fill: gradient.radial(green.lighten(80%), green, center: (30%, 20%), radius: 80%)),
    node((1,0), "Value [-1]", radius: 2em),
    node((-1,0), "Value [-6]", radius: 2em),
    node((1,-1), "Value [-2]", radius: 2em),
    node((1,-2), "Value [-2]", radius: 2em),
    node((0,-2), "Value [-3]", radius: 2em),
    node((-1,-2), "Value [-4]", radius: 2em),
    node((-1,-1), "Value [-5]", radius: 2em),
    edge((0,2), (0,1), "u", "-|>", `Value on top of the stack`, label-pos: 0, label-side: right),
  )
]
#set text(13pt)
#pagebreak()
=== Stack-of-stacks references
\
\
The next level of abstraction is a circular stack that refers to named data stacks while functioning just like a standard data stack in all other aspects. The stack referred to by the "top of the stack" reference is considered the "current stack," and all operations are by default performed within this data context. When creating a new stack, the reference moves to the top of the stack. When positioning a named stack to become the current stack, the buffer rotates to bring the required stack to the proper position at the "top of the stack."
\
\
\
\
\
#set text(9pt)
#align(center)[
  #diagram(
    node-stroke: .1em,
    node-fill: gradient.radial(blue.lighten(80%), blue, center: (30%, 20%), radius: 80%),
    spacing: 2em,
    node((0,0), "Stack A", radius: 2em, fill: gradient.radial(green.lighten(80%), green, center: (30%, 20%), radius: 80%)),
    node((1,0), "Stack B", radius: 2em),
    node((-1,0), "Stack H", radius: 2em),
    node((1,-1), "Stack C", radius: 2em),
    node((1,-2), "Stack D", radius: 2em),
    node((0,-2), "Stack E", radius: 2em),
    node((-1,-2), "Stack F", radius: 2em),
    node((-1,-1), "Stack G", radius: 2em),
    edge((0,2), (0,1), "u", "-|>", `Current stack`, label-pos: 0, label-side: right),
  )
]
#set text(13pt)

#pagebreak()
=== Workbench
\
\
The workbench, an integral component of the BUND virtual machine, is a circular stack that temporarily holds and transfers values between computations conducted in various data contexts. Despite its functional significance, this circular stack does not carry a specific name.
\
