#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Bund-Eval", args: (), {
    Cmt[
      Evaluate bund code snippet
    ]
    Assign(
      [Snippet], [_current stack_]
    )
    If(cond: "Snippet = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Eval", [Snippet])
  })
})
