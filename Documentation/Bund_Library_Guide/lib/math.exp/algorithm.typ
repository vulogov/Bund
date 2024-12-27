#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("math.exp", args: (), {
    Cmt[
      E to the power of X
    ]
    Assign(
      [X], [_current stack_]
    )
    If(cond: "X = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Math_Exp", [X])]
    )
  })
})
