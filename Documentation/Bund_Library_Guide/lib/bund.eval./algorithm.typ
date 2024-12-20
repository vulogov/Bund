#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Bund-Eval-from-Workbench", args: (), {
    Cmt[
      Evaluate bund code snippet
    ]
    Assign(
      [Snippet], [_workbench_]
    )
    If(cond: "Snippet = None", {
      Return[Error("Workbench is too shallow")]
    })
    Call("Bund_Eval", [Snippet])
  })
})
