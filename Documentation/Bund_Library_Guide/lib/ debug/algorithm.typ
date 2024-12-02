#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Debug_Shell", args: (), {
    Cmt[
      Bund debugger
    ]
    Assign(
      [Snippet], [_current stack_]
    )
    If(cond: "Snippet = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Debugger", [Snippet])
  })
})
