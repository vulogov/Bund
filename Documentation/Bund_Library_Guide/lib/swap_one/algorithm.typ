#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Swap_One", args: (), {
    Cmt[
      Swapping data in stack
    ]
    Assign(
      [Value1], [_current stack_]
    )
    If(cond: "Value1 = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Value2], [_current stack_]
    )
    If(cond: "Value2 = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Value2]
    )
    Assign(
      [_current stack_], [Value1]
    )
  })
})
