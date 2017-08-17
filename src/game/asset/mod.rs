pub mod mesh;

pub use self::mesh::Mesh;

#[derive(PartialEq, Debug, Clone)]
pub enum LoadingState {
    Unloaded,
    Loaded,
}
