#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("math.power", args: (), {
    Cmt[
      n-th root of X
    ]
    Assign(
      [N], [_current stack_]
    )
    If(cond: "N = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [X], [_current stack_]
    )
    If(cond: "X = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Math_Power", [X, N])]
    )
  })
})
