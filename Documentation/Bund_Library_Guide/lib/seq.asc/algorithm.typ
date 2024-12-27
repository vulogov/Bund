#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("seq.asc", args: (), {
    Cmt[
      Creating sequence of float numbers
    ]
    Assign(
      [X], [_current stack_]
    )
    If(cond: "X = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [Step], [_current stack_]
    )
    If(cond: "Step = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [N], [_current stack_]
    )
    If(cond: "N = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Seq_Asc", [X, Step, N])]
    )
  })
})
