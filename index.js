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
          refNames: [],
          message: "",
          parents: ["hash3"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
        {
          hash: "hash1+",
          refNames: [],
          message: "",
          parents: ["hash2+"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
        {
          hash: "hash3",
          refNames: [],
          message: "",
          parents: ["hash4"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
        {
          hash: "hash1++",
          refNames: [],
          message: "",
          parents: ["hash4"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
        {
          hash: "hash2+",
          refNames: [],
          message: "",
          parents: ["hash4"],
          commitDate: "",
          authorEmail: "",
          authorName: "",
          authorDate: "",
        },
        {
          hash: "hash4",
          refNames: [],
          message: "",
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
