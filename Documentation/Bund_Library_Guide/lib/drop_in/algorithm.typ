#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("Drop_in", args: (), {
    Cmt[
      Drop the value in the named stack
    ]
    Assign(
      [Name], [VM::current_stack_name()]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Drop", [Name])
  })
})
