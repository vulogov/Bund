#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Dup_One_in", args: (), {
    Cmt[
      Duplicate value
    ]
    Assign(
      [Name], [_current stack_]
    )
    Assign(
      [Value], [_current stack_]
    )
    Assign(
      [_stack Name_], [Call("Dup", [1, Name, Value])]
    )
  })
})
