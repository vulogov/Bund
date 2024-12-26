#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Writing string representation of the data to the file
    ]
    Assign(
      [Filename], [_current stack_]
    )
    If(cond: "Filename = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Data], [_workbench_]
    )
    If(cond: "Data = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("File_Write_Workbench", [Filename, Call("To_String", [Data])])
  })
})
