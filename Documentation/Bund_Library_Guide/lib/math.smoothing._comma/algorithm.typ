#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("math.smoothing.,", args: (), {
    Cmt[
      SMA smoothing
    ]
    Assign(
      [Sample], [_workbench_]
    )
    If(cond: "Sample = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_workbench_], [Sample]
    )
    Assign(
      [_workbench_], [Call("Math_Smoothing", [Sample])]
    )
  })
})
