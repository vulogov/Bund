#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Move value between named stacks
    ]
    Assign(
      [Name_From], [_current stack_]
    )
    If(cond: "Name_From = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Name_To], [_current stack_]
    )
    If(cond: "Name_To = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Value], [_Name_From stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_Name_To stack_], [Value]
    )
  })
})
