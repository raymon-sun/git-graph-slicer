const rust = import("./pkg");
rust.then((m) => console.log(m.attach_graph([]))).catch(console.error);