#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Rotate_Current_Left", args: (), {
    Cmt[
      Rotate named stack
    ]
    Assign(
      [Name], [_current stack_]
    )
    Call("rotate_stack_right", [1, Name])
  })
})
