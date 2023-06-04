use std::{ops::Deref, sync::Mutex};

use refs::Weak;

use crate::View;

struct Subscriber<T: Send> {
    view:   Weak<dyn View>,
    action: Box<dyn FnMut(T) + Send>,
}

#[derive(Default)]
pub struct UIEvent<T: Send = ()> {
    subscribers: Mutex<Vec<Subscriber<T>>>,
}

impl<T: Send> UIEvent<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(vec![]),
        }
    }

    pub fn sub(
        &self,
        view: impl Deref<Target = impl View + ?Sized>,
        mut action: impl FnMut() + Send + 'static,
    ) {
        self.subscribers.lock().unwrap().push(Subscriber {
            view:   view.weak_view(),
            action: Box::new(move |_| action()),
        })
    }

    pub fn val(&self, view: impl Deref<Target = impl View + ?Sized>, action: impl FnMut(T) + Send + 'static) {
        self.subscribers.lock().unwrap().push(Subscriber {
            view:   view.weak_view(),
            action: Box::new(action),
        })
    }

    pub fn trigger(&self, val: T)
    where T: Copy {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|a| a.view.is_ok());
        for sub in subs.iter_mut() {
            (sub.action)(val)
        }
    }
}