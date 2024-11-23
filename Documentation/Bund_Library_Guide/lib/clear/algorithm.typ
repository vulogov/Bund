#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("function-name", args: (), {
    Cmt[
      Clearing all values in current stack
    ]
    Assign(
      [Name], [VM::current_stack_name()]
    )
    If(cond: "Name = None", {
      Return[Error("Error getting current stack name")]
    })
    Call("Clear_Stack", [Name])
  })
})
