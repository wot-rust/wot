#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sifis-home/wot/master/assets/wot-rust.svg"
)]
//! Web of Things (in rust)
//!
//! This crate collects a number of subcrates that cover different aspects of [Web of
//! Things][architecture].
//!
//! The normal usage is to directly depend on the specific crates you may need to use:
//!
//! - [wot-td](https://crates.io/crates/wot-td)
//! - [wot-serve](https://crates.io/crates/wot-serve)
//!
//! [architecture]: https://www.w3.org/TR/wot-architecture11/

#[doc(inline)]
pub use wot_serve as serve;
#[doc(inline)]
pub use wot_td as td;

pub use serve::Servient;
pub use td::Thing;
