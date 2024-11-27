#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Ensure_Stack", args: (), {
    Cmt[
      Make stack current, create if not exists
    ]
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    If(cond: "Not Call(Stack_Exists, [Name])", {
      Call("Create_Stack", [Name])
    })
    Call("To_Stack", [Name])
  })
})
