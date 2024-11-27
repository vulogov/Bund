#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Rotate_Current_Left", args: (), {
    Cmt[
      Rotate current stack
    ]
    Call("rotate_current_left", [1])
  })
})
