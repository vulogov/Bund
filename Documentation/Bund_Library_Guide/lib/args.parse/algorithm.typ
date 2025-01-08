#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Return parsed arguments (if any)
    ]
    Assign(
      [_current stack_], [Call("Args.Parse", [ARGS])]
    )
  })
})
