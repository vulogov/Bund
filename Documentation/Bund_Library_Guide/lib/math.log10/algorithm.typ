#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("math.log10", args: (), {
    Cmt[
      Base 10 logarithm of X
    ]
    Assign(
      [X], [_current stack_]
    )
    If(cond: "X = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Math_Log10", [X])]
    )
  })
})
