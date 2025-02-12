#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("drop", args: (), {
    Cmt[
      Dropping value that is on top of the stack
    ]
    Assign(
      [Name], [VM::current_stack_name()]
    )
    If(cond: "Value = None", {
      Return[Error("Stack is too shallow")]
    })
    Call("Drop", [Name])
  })
})
