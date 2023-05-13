#![cfg(desktop)]

use std::{marker::PhantomData, path::PathBuf};

use gm::flat::Size;
use ui::{
    refs::{Own, ToWeak},
    View, ViewTest,
};

use crate::{app::MakeApp, App, AppCore};

pub struct ViewApp<T> {
    core: AppCore,
    _v:   PhantomData<*const T>,
}

impl<T: View + Default + 'static> ViewApp<T> {
    pub fn start() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            Self::make_app().launch();
        });
    }
}

impl<T: View + Default + 'static> App for ViewApp<T> {
    fn screen_size() -> Size
    where Self: Sized {
        T::test_size()
    }

    fn make_root_view() -> Own<dyn View>
    where Self: Sized {
        let view = Own::<T>::default();
        view.weak().test_setup();
        view
    }

    fn with_core(core: AppCore) -> Self
    where Self: Sized {
        Self {
            core,
            _v: Default::default(),
        }
    }

    fn core(&mut self) -> &mut AppCore {
        &mut self.core
    }

    fn assets_path() -> PathBuf
    where Self: Sized {
        Default::default()
    }
}

#[macro_export]
macro_rules! test_view {
    ($view_type:ty) => {
        #[ignore]
        #[test]
        fn test_view() {
            ViewApp::<$view_type>::start();
        }
    };
}

#[ignore]
#[test]
fn test() {
    ViewApp::<ui::Container>::start()
}
