#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Drop_Stack", args: (), {
    Cmt[
      Drop the stack
    ]
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("DropStack", [Name])
  })
})
