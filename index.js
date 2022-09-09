const rust = import("./pkg");
rust
  .then((m) =>
    console.log(
      // m.attach_graph({
      //   field2: [
      //     [1, 2],
      //     [3, 4],
      //   ],
      //   field3: [1, 2, 3, 4],
      // })
      m.attach_graph([
        {
          hash: "de",
          refNames: ["123"],
          message: "",
          parents: ["123"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
      ])
    )
  )
  .catch(console.error);
