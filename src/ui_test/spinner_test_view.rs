use rtools::sleep;
use ui::{refs::Weak, view, SubView, ViewSetup};
use ui_views::{async_link_button, Button, Spinner};

#[view]
struct SpinnerTestView {
    button: SubView<Button>,
}

impl SpinnerTestView {
    async fn tap(self: Weak<Self>) {
        Spinner::start();
        sleep(3);
        Spinner::stop();
    }
}

impl ViewSetup for SpinnerTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Spin");
        async_link_button!(self, button, tap);
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<SpinnerTestView>::start();
}
