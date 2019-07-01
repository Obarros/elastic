use std::fs::File;

fn load_file(p: &str) -> File {
    File::open(format!("src/client/responses/tests/samples/{}", p)).unwrap()
}

mod bulk;
mod command;
mod document_delete;
mod document_get;
mod document_index;
mod document_update;
mod index_exists;
mod nodes_info;
mod ping;
mod search;
