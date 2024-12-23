#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-load-aliases", args: (), {
    Cmt[
      Restore VM function aliases from WORLD file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Load_Aliases", [Filename])
  })
})
