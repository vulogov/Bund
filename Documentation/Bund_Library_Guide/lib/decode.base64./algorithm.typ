#import "@preview/algorithmic:0.1.0"
#import algorithmic: algorithm

#algorithm({
  import algorithmic: *
  Function("decode-base64-workbench", args: (), {
    Cmt[
      Decoding from BASE64
    ]
    Assign(
      [Data], [_workbench_]
    )
    If(cond: "Data = None", {
      Return[Error("Workbench is too shallow")]
    })
    Assign(
      [_workbench_], [Call("Decode_Base64", [Data])]
    )
  })
})
