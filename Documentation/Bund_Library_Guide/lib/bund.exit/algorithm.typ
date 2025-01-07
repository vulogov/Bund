#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Terminate BUND interpreter
    ]
    Assign(
      [Exit_code], [_current stack_]
    )
    If(cond: "Value = None", {
      Call("Exit", [0])
    })
    Call("Exit", [Exit_code])
  })
})
