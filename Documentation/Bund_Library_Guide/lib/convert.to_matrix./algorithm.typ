#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Convert_To_Matrix_In_Workbench", args: (), {
    Cmt[
      Converting Value to Matrix
    ]
    Assign(
      [Value], [_workbench_]
    )
    If(cond: "Value = None", {
      Return[Error("Workbench is too shallow")]
    })
    Assign(
      [_workbench_], [Value::conv(MATRIX)]
    )
  })
})