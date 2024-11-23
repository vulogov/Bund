#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Dup_Many", args: (), {
    Cmt[
      Duplicate multiple values
    ]
    Assign(
      [N], [_current stack_]
    )
    Assign(
      [Name], [VM::current_stack_name()]
    )
    While(cond: "N >= 0", {
      Assign(
        [Value], [_current stack_]
      )
      Assign(
        [_current stack_], [Call("Dup", [N, Name, Value])]
      )
      Assign(
        [N], [N - 1]
      )
    })
  })
})
