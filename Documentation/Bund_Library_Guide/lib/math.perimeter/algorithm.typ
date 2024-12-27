#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("math.perimeter", args: (), {
    Cmt[
      Compute length of rectangle boundaries
    ]
    Assign(
      [X], [_current stack_]
    )
    If(cond: "N = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Y], [_current stack_]
    )
    If(cond: "X = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Math_Perimeter", [X, Y])]
    )
  })
})
