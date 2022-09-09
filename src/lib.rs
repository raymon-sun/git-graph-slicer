use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Commit {
    hash: String,
    ref_names: Vec<String>,
    message: String,
    parents: Vec<String>,
    commit_date: String,
    author_email: String,
    author_name: String,
    author_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphicCommit {
    hash: String,
    ref_names: Vec<String>,
    message: String,
    parents: Vec<String>,
    commit_date: String,
    author_email: String,
    author_name: String,
    author_date: String,
    graph: GraphSlice,
}

#[derive(Serialize, Deserialize)]
struct GraphSlice {
    commit_position: u32,
    commit_color: String,
    lines: Vec<CommitGraphLine>,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct CommitGraphLine {
    top: u32,
    bottom: u32,
    color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
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
    let original_commits: Vec<Commit> = serde_wasm_bindgen::from_value(val).unwrap();

    let mut pre_lines: Vec<CommitGraphLine> = Vec::new();
    let commits: Vec<GraphicCommit> = original_commits
        .into_iter()
        .map(|commit| {
            let hash = &commit.hash;
            let parents = &commit.parents;
            let graph_slice = slice_graph(hash, parents, &pre_lines);
            pre_lines = graph_slice.lines.clone();

            GraphicCommit {
                hash: hash.clone(),
                ref_names: commit.ref_names,
                message: commit.message,
                parents: parents.clone(),
                commit_date: commit.commit_date,
                author_email: commit.author_email,
                author_name: commit.author_name,
                author_date: commit.author_date,
                graph: graph_slice,
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&commits).unwrap()
}

fn slice_graph(hash: &String, parents: &Vec<String>, pre_lines: &Vec<CommitGraphLine>) -> GraphSlice {
    GraphSlice {
        commit_position: 0,
        commit_color: "".to_string(),
        lines: Vec::new(),
    }
}
