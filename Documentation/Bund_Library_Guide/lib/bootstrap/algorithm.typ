#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-bootstrap", args: (), {
    Cmt[
      Load and execute bootstrap scripts from the WORLD file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Bootstrap", [Filename])
  })
})
