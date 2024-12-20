#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Return passed arguments (if any)
    ]
    Assign(
      [_current stack_], [ARGS]
    )
  })
})
