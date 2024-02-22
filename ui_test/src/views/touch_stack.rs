use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    ui::{view, Alert, Button, SubView, TouchStack, ViewSubviews, ViewTouch},
    wait_for_next_frame, App,
};

use crate::view_tests::{assert_eq, inject_touches};

#[view]
struct TouchStackTestView {
    #[text = a]
    button:  SubView<Button>,
    #[text = b]
    button2: SubView<Button>,
}

pub async fn test_touch_stack() -> Result<()> {
    let mut view = App::set_test_view::<TouchStackTestView>(600, 600).await;

    assert_eq(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]])?;

    let mut button = from_main(move || view.add_view::<Button>()).await;

    assert_eq(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]])?;

    button.on_tap(|| {});

    assert_eq(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            "View: ".to_string() + &button.label.clone(),
        ]],
    )?;

    from_main(move || button.remove_from_superview()).await;

    wait_for_next_frame().await;

    assert_eq(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]])?;

    view.button.on_tap(|| {});

    assert_eq(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            "View: ".to_string() + &view.button.label.clone(),
        ]],
    )?;

    view.button2.on_tap(|| {});

    assert_eq(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view".to_string(),
            "View: ".to_string() + &view.button.label.clone(),
            "View: ".to_string() + &view.button2.label.clone(),
        ]],
    )?;

    view.button.disable_touch();
    view.button2.disable_touch();

    assert_eq(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]])?;

    Alert::show("Hello");

    wait_for_next_frame().await;

    assert_eq(
        TouchStack::dump(),
        vec![
            vec!["Layer: Root view".to_string()],
            vec![
                "Layer: Alert".to_string(),
                "View: Alert.ok_button: Button".to_string(),
            ],
        ],
    )?;

    inject_touches(
        r#"
            320  383  b
            320  383  e
    "#,
    )
    .await;

    assert_eq(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]])?;

    debug!("Touch stack test: OK");

    Ok(())
}
