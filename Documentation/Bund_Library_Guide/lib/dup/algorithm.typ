#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Dup", args: (), {
    Cmt[
      Duplicate value
    ]
    Assign(
      [Value], [_current stack_]
    )
    Assign(
      [_current stack_], [Call("Dup", [Value])]
    )
  })
})
