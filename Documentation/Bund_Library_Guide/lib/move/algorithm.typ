#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Move value from current stack
    ]
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Value], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_Name stack_], [Value]
    )
  })
})
