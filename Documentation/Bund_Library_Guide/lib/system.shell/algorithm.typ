#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("system.shell", args: (), {
    Cmt[
      Execute an external command
    ]
    Assign(
      [Cmd], [_current stack_]
    )
    If(cond: "Cmd = None", {
      Return[Error("Stack is too shallow")]
    })
  })
})
