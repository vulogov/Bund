#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Convert_To_String", args: (), {
    Cmt[
      Converting Value to String
    ]
    Assign(
      [Value], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Value::conv(STRING)]
    )
  })
})
