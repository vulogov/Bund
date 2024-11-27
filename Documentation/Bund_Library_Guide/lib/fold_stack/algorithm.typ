#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Fold", args: (), {
    Cmt[
      Folding data in the named stack
    ]
    Assign(
      [Result], [[]]
    )
    Assign(
      [Name], [_current stack_]
    )
    For(cond: "Name not Empty", {
      Assign(
        [Value], [_Name stack_]
      )
      If(cond: "Value = NODATA", {
        Call("Break", [])
      })
      Assign(
        [Result], [Value]
      )
    })
    Assign(
      [_Name stack_], [Result]
    )
  })
})
