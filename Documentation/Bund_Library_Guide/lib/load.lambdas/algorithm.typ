#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-load-lambdas", args: (), {
    Cmt[
      Restore lambda functions from WORLD file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Load_Lambdas", [Filename])
  })
})
