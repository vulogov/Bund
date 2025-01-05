#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("id.uuid", args: (), {
    Cmt[
      Generate UUID
    ]
    Assign(
      [_current_stack_], [Call("Uuid.new_v4", [])]
    )
  })
})
