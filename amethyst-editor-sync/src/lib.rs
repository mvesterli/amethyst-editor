extern crate amethyst;
#[macro_use]
extern crate serde;
extern crate serde_json;

mod sync_editor;

pub use sync_editor::SyncEditorSystem;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct EntityData {
    id: u32,
    generation: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message<T> {
    #[serde(rename = "type")]
    ty: &'static str,
    data: T,
}
