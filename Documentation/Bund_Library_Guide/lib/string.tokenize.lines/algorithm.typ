#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("string.tokenize.lines.stack", args: (), {
    Cmt[
      String tokenization
    ]
    Assign(
      [Text], [_current stack_]
    )
    If(cond: "Text = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("String.Tokenize.Lines", [Text])]
    )
  })
})
