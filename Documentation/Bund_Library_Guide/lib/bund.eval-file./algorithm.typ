#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Bund-Eval-File-Dot", args: (), {
    Cmt[
      Evaluate bund code snippet
    ]
    Assign(
      [Snippet], [Call("Read", [_workbench_])]
    )
    If(cond: "Snippet = None", {
      Return[Error("Workbench is too shallow")]
    })
    Call("Bund_Eval", [Snippet])
  })
})
