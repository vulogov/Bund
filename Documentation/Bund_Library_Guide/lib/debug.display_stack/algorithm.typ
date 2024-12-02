#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Debug_Display_Stack", args: (), {
    Cmt[
      Display stack state
    ]
    Call("Debug_Display_Stack", [])
  })
})
