#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("encode-base64", args: (), {
    Cmt[
      Encoding to BASE64
    ]
    Assign(
      [Data], [_workbench_]
    )
    If(cond: "Data = None", {
      Return[Error("Stack is too shallow")]
    })
    Assign(
      [_workbench_], [Call("Encode_Base64", [Data])]
    )
  })
})
