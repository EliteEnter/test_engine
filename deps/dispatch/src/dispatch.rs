use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use refs::is_main_thread;
use rtools::{sleep, IntoF32};
use tokio::{
    spawn,
    sync::oneshot::{channel, Sender},
};

type Callbacks = Mutex<Vec<Box<dyn FnOnce() + Send>>>;
type SignalledCallbacks = Mutex<Vec<(Sender<()>, Box<dyn FnOnce() + Send>)>>;

static CALLBACKS: Callbacks = Callbacks::new(vec![]);
static SIGNALLED: SignalledCallbacks = SignalledCallbacks::new(vec![]);

pub async fn from_main<T, A>(action: A) -> T
where
    A: FnOnce() -> T + Send + 'static,
    T: Send + 'static, {
    assert!(
        !is_main_thread(),
        "This is already main thread. Just call it without `from_main`"
    );

    let res = Arc::<Mutex<Option<T>>>::default();

    let (sender, receiver) = channel::<()>();

    let capture = res.clone();
    SIGNALLED.lock().unwrap().push((
        sender,
        Box::new(move || {
            let mut res = capture.lock().unwrap();
            *res = action().into();
        }),
    ));

    receiver.await.unwrap();

    let res = res.lock().unwrap().take().unwrap();
    res
}

pub fn on_main(action: impl FnOnce() + Send + 'static) {
    if is_main_thread() {
        action();
    } else {
        CALLBACKS.lock().unwrap().push(Box::new(action));
    }
}

pub fn on_main_sync(action: impl FnOnce() + Send + 'static) {
    if is_main_thread() {
        action();
    } else {
        let (sender, mut receiver) = channel::<()>();
        SIGNALLED.lock().unwrap().push((sender, Box::new(action)));
        while receiver.try_recv().is_err() {}
    }
}

pub fn after(delay: impl IntoF32, action: impl FnOnce() + Send + 'static) {
    spawn(async move {
        sleep(delay);
        CALLBACKS.lock().unwrap().push(Box::new(action));
    });
}

pub fn async_after(delay: impl IntoF32, action: impl Future + Send + 'static) {
    spawn(async move {
        sleep(delay);
        action.await;
    });
}

pub fn invoke_dispatched() {
    let mut callback = CALLBACKS.lock().unwrap();
    for action in callback.drain(..) {
        action()
    }
    drop(callback);

    let mut signalled = SIGNALLED.lock().unwrap();
    for action in signalled.drain(..) {
        (action.1)();
        action.0.send(()).unwrap();
    }
}
