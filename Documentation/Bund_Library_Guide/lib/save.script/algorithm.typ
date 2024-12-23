#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("bund-save-script", args: (), {
    Cmt[
      Saving bootstrap script into the WORLD file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Filename = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Snippet], [_current stack_]
    )
    If(cond: "Snippet = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Bund_Save_Script", [Filename, Snippet, Name])
  })
})
