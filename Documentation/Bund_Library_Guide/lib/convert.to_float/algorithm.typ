#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Convert_To_Float", args: (), {
    Cmt[
      Converting Value to Float
    ]
    Assign(
      [Value], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Value::conv(FLOAT)]
    )
  })
})
