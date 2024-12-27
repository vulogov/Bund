#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("math.factorial", args: (), {
    Cmt[
      factorial of X
    ]
    Assign(
      [X], [_current stack_]
    )
    If(cond: "X = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Math_Factorial", [X])]
    )
  })
})
