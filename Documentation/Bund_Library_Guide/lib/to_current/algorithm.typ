#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("to_current", args: (), {
    Cmt[
      Make already existing stack a current stack
    ]
    Assign(
      [Value], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("To_Current", [Name])
  })
})
