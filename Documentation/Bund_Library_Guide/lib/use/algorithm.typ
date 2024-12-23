#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-use-from-stack", args: (), {
    Cmt[
      Load and execute external scripts
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Use", [Filename])
  })
})
