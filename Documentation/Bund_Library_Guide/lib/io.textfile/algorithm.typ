#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("io.textfile", args: (), {
    Cmt[
      Reading text file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Io.Textfile", [Filename])]
    )
  })
})
