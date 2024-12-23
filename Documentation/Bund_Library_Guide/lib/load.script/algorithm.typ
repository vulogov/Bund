#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-load-script", args: (), {
    Cmt[
      Loading bootstrap script from the WORLD file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Filename = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Bund_Load_Script", [Filename, Snippet, Name])]
    )
  })
})
