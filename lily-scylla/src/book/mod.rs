mod create;
// mod schema;
// mod db;
mod read;
mod update;
mod delete;
mod image;

pub use image::upload_image;
pub use create::{create_new_book, create_new_chapter, create_new_section, create_new_page};
pub use read::{get_all, get_one, get_all_from_id};
pub use update::update_one;
pub use delete::delete_one;