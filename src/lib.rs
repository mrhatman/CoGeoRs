//!Computational Geometry Algorithms in Rust

#![allow(dead_code)]
#![warn(missing_docs)]

pub use crate::{
    primatives2d::{Point2D,Line2D},
    dcel::DCEL,
};

mod primatives2d;
mod dcel;
mod dcelRC;
pub mod algorithms;


