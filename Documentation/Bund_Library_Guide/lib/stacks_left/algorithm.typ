#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Stacks_Left", args: (), {
    Cmt[
      Rotate list of stacks
    ]
    Call("stacks_left", [1])
  })
})
