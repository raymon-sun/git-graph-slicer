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

#[derive(Clone, Serialize, Deserialize)]
struct CommitGraphLine {
    top: i32,
    bottom: i32,
    color: String,
}
#[derive(Clone, Serialize, Deserialize)]
struct Chain {
    hash: String,
    parent: String,
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

    let cur_chains: Vec<Chain> = Vec::new();
    let mut pre_lines: Vec<CommitGraphLine> = Vec::new();
    let commits: Vec<GraphicCommit> = original_commits
        .into_iter()
        .map(|commit| {
            let hash = &commit.hash;
            let parents = &commit.parents;
            let graph_slice = slice_graph(hash, parents, &pre_lines, &cur_chains);
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

fn slice_graph(
    hash: &String,
    parents: &Vec<String>,
    pre_lines: &Vec<CommitGraphLine>,
    chains: &Vec<Chain>,
) -> GraphSlice {
    let lines = get_current_lines(pre_lines);

    let firstParent = &parents[0];
    let forkParents = &parents[1..];

    let index_list = get_index_list(chains, hash);

    GraphSlice {
        commit_position: 0,
        commit_color: "".to_string(),
        lines,
    }
}

fn get_current_lines(pre_lines: &Vec<CommitGraphLine>) -> Vec<CommitGraphLine> {
    let mut existed_bottoms: Vec<i32> = Vec::new();

    pre_lines
        .iter()
        .filter(|line| {
            if existed_bottoms.contains(&line.bottom) {
                return false;
            }

            existed_bottoms.push(line.bottom);
            line.bottom != -1i32
        })
        .map(|line| CommitGraphLine {
            top: line.bottom,
            bottom: line.bottom,
            color: line.color.clone(),
        })
        .collect::<Vec<CommitGraphLine>>()
}

fn get_index_list(chains: &Vec<Chain>, hash: &String) -> Vec<u32> {
    let mut index_list: Vec<u32> = Vec::new();

    chains.into_iter().enumerate().for_each(|(index, chain)| {
        if chain.parent.eq(hash) {
            index_list.push(index as u32);
        }
    });

    index_list
}
