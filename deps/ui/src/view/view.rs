use std::fmt::Debug;

use rtools::{Boxed, Rglica};

use crate::{ViewBase, ViewCallbacks};

pub trait View: Boxed + Debug + ViewCallbacks {
    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
    fn rglica(&self) -> Rglica<dyn View>;
}

#[macro_export]
macro_rules! impl_view {
    ($type:ident) => {
        impl View for $type {
            fn view(&self) -> &ViewBase {
                &self.view
            }
            fn view_mut(&mut self) -> &mut ViewBase {
                &mut self.view
            }
            fn rglica(&self) -> Rglica<dyn View> {
                (self as &dyn View).to_rglica()
            }
        }
    };
}
