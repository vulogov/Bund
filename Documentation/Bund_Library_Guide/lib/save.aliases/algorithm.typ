#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-save-aliases", args: (), {
    Cmt[
      Restore VM state from WORLD file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Save_Aliases", [Filename])
  })
})
