use std::path::PathBuf;

use audio::Sound;
use gl_image::Image;
use manage::data_manager::DataManager;
use text::Font;
use ui::refs::assert_main_thread;

use crate::paths::Paths;

pub struct Assets;

impl Assets {
    pub fn init(root_path: impl Into<PathBuf>) {
        assert_main_thread();

        let paths = Paths::new(root_path.into());

        Image::set_root_path(&paths.images);
        Sound::set_root_path(&paths.sounds);
        Font::set_root_path(&paths.fonts);
    }
}
