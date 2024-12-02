#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Debug_Shell", args: (), {
    Cmt[
      Drop into a debug shell
    ]
    Call("Debug_Shell", [])
  })
})
