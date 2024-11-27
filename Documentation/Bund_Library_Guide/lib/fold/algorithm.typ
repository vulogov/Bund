#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Fold", args: (), {
    Cmt[
      Folding data in the current stack
    ]
    Assign(
      [Result], [[]]
    )
    For(cond: "Current Stack not Empty", {
      Assign(
        [Value], [_current stack_]
      )
      If(cond: "Value = NODATA", {
        Call("Break", [])
      })
      Assign(
        [Result], [Value]
      )
    })
    Assign(
      [_current stack_], [Result]
    )
  })
})
