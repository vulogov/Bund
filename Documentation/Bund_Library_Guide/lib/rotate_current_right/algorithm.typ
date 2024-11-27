#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Rotate_Current_Right", args: (), {
    Cmt[
      Rotate current stack
    ]
    Call("rotate_current_right", [1])
  })
})
