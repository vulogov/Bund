#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("decode-base64", args: (), {
    Cmt[
      Decoding from BASE64
    ]
    Assign(
      [Data], [_current stack_]
    )
    If(cond: "Data = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_current stack_], [Call("Decode_Base64", [Data])]
    )
  })
})
