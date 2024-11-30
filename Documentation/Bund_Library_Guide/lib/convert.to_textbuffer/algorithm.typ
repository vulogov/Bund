#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Convert_To_Textbuffer", args: (), {
    Cmt[
      Converting Value to Textbuffer
    ]
    Assign(
      [Value], [_current stack_]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Value::conv(TEXTBUFFER)]
    )
  })
})
