#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("clear_in", args: (), {
    Cmt[
      Clearing named stack
    ]
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Clear_Stack", [Name])
  })
})
