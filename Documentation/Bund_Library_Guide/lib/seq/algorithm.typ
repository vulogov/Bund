#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("seq", args: (), {
    Cmt[
      Creating sequence of float numbers
    ]
    Assign(
      [Conf], [_current stack_]
    )
    If(cond: "Conf = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Seq", [Conf])]
    )
  })
})
