#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Rotate_Current_Left", args: (), {
    Cmt[
      Rotate current stack
    ]
    Assign(
      [Name], [VM::current_stack_name()]
    )
    Call("rotate_stack_left", [1, Name])
  })
})
