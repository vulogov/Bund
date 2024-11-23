#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Dup_One", args: (), {
    Cmt[
      Duplicate value
    ]
    Assign(
      [Value], [_current stack_]
    )
    Assign(
      [Name], [VM::current_stack_name()]
    )
    Assign(
      [_current stack_], [Call("Dup", [1, Name, Value])]
    )
  })
})
