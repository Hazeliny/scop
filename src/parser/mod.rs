pub mod obj_parser;		// → loads src/parser/obj_parser.rs

// Re-export for convenience
pub use obj_parser::{parse, Mesh, Vertex};