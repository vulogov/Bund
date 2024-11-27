#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Stack_Exists", args: (), {
    Cmt[
      Check if stack exists
    ]
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    If(cond: "Not Call(Stack_Exists, [Name])", {
      Assign(
        [_current stack_], [FALSE]
      )
    })
    Else({
    Assign(
      [_current stack_], [TRUE]
    )
    })
  })
})
