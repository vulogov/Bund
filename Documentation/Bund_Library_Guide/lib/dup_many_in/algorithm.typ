#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Dup_Many_In", args: (), {
    Cmt[
      Duplicate multiple items in the named stack
    ]
    Assign(
      [Name], [_current stack_]
    )
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    If(cond: "N = None", {
      Return[Error("Stack is too shallow")]
    })
    While(cond: "N >= 0", {
      Assign(
        [Value], [_Name stack_]
      )
      Assign(
        [_Name stack_], [Call("Dup", [N, Name, Value])]
      )
      Assign(
        [N], [N - 1]
      )
    })
  })
})
