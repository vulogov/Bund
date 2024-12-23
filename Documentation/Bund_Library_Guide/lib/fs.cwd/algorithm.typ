#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("fs-cwd", args: (), {
    Cmt[
      Return CWD
    ]
    Assign(
      [_current stack_], [Call("Fs_Cwd", [])]
    )
  })
})
