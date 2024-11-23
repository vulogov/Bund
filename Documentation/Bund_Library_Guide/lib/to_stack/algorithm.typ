#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("to_current", args: (), {
    Cmt[
      Make stack a current stack. Create if not existing.
    ]
    Assign(
      [Name], [_current stack_]
    )
    If(cond: "Name = None", {
      Return[Error("Stack is too shallow")]
    })
    If(cond: "not VM::stack_exists(Name)", {
      Call("Ensure_Stack", [Name])
    })
    Call("To_Current", [Name])
  })
})
