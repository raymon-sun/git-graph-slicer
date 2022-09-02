use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct Commit {
    hash: String,
    ref_names: Vec<String>,
    message: String,
    parents: Vec<String>,
    commit_date: String,
    author_email: String,
    author_name: String,
    author_date: String,
    graph: Vec<GraphSlice>,
}

#[derive(Serialize, Deserialize)]
struct GraphSlice {
    commit_position: u32,
    commit_color: String,
    lines: Vec<CommitGraphLine>,
}

#[derive(Serialize, Deserialize)]
struct CommitGraphLine {
    top: u32,
    bottom: u32,
    color: String,
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hi, {}!", name));
}

#[wasm_bindgen]
pub fn attach_graph(val: JsValue) -> JsValue {
    let commits: Vec<Commit> = serde_wasm_bindgen::from_value(val).unwrap();

    commits.iter().map(|commit| attach_graph_slice(commit));

    serde_wasm_bindgen::to_value(&commits).unwrap()
}

fn attach_graph_slice(original_commit: &Commit) -> Commit {
    let commit =  Commit {
        hash: original_commit.hash.clone(),
        ref_names: original_commit.ref_names.clone(),
        message: original_commit.message.clone(),
        parents: original_commit.parents.clone(),
        commit_date: original_commit.commit_date.clone(),
        author_email: original_commit.author_email.clone(),
        author_name: original_commit.author_name.clone(),
        author_date: original_commit.author_date.clone(),
        graph : vec![GraphSlice {
            commit_position: 0,
            commit_color: "".to_string(),
            lines: vec![],
        }]
    };

    commit
}
