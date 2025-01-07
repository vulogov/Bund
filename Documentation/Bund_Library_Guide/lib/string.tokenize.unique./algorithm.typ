#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("string.tokenize.unique.wb", args: (), {
    Cmt[
      String tokenization
    ]
    Assign(
      [Text], [_workbench_]
    )
    If(cond: "Text = None", {
      Return[Error("Workbench is too shallow")]
    })
    Assign(
      [_workbench_], [Call("String.Tokenize.Unique", [Text])]
    )
  })
})
