mod error;
mod mojang;
mod mineskin;

use once_cell::sync::OnceCell;
use reqwest::{Client, ClientBuilder};

pub(crate) use error::Result;
pub use error::Error;
pub use mojang::Mojang;
pub use mineskin::Mineskin;

const CLIENT: OnceCell<Client> = OnceCell::new();

pub(crate) fn client() -> Client {
    CLIENT.get_or_init(|| ClientBuilder::new().user_agent("SkinFixer Spigot Plugin API").build().unwrap()).clone()
}