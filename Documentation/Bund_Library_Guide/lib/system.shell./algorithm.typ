#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("system.shell.wb", args: (), {
    Cmt[
      Execute an external command
    ]
    Assign(
      [Cmd], [_workbench_]
    )
    If(cond: "Cmd = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_workbench_], [Call("System.Shell", [Cmd])]
    )
  })
})
