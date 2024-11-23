#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("to_current", args: (), {
    Cmt[
      Returns the name of current stack to stack
    ]
    Assign(
      [Name], [VM::current_stack_name()]
    )
    If(cond: "Name = None", {
      Return[Error("Can not detect current stack name")]
    })
    Assign(
      [_current stack_], [Name]
    )
  })
})
