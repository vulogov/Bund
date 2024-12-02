#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Debug_Display_Workbench", args: (), {
    Cmt[
      Display Workbench State
    ]
    Call("Debug_Display_Workbench", [])
  })
})
