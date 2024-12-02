#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Debug_Display_Hostinfo", args: (), {
    Cmt[
      Display current Host information
    ]
    Call("Debug_Display_Hostinfo", [])
  })
})
