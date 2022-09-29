const rust = import("./pkg");
rust
  .then((m) =>
    console.log(
      m.attach_graph([
        {
          hash: "hash1",
          refNames: ["origin/dev"],
          message: "care",
          parents: ["hash2"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
        {
          hash: "hash2",
          refNames: [""],
          message: "care too much",
          parents: [],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
      ])
    )
  )
  .catch(console.error);
