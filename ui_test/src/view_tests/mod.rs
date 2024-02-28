use std::{
    fmt::{Debug, Display},
    ops::Deref,
    sync::{Arc, Mutex},
};

use anyhow::{bail, Result};
use log::{error, warn};
use serde::de::DeserializeOwned;
use test_engine::{
    from_main,
    gm::LossyConvert,
    on_main,
    refs::ToOwn,
    sleep,
    ui::{Touch, U8Color, UIEvents, UIManager},
    App,
};
use tokio::sync::mpsc::channel;

use crate::view_tests::state::{clear_state, get_state};

const INJECT_TOUCH_DELAY: f32 = 0.0;

pub mod state;

pub async fn test_combinations<const A: usize, Val>(comb: [(&'static str, Val); A]) -> Result<()>
where Val: Display + PartialEq + DeserializeOwned + Default + Send + 'static {
    for comb in comb {
        clear_state();

        let touches = Touch::vec_from_str(comb.0);

        for touch in touches {
            inject_touch(touch).await;
        }

        if get_state::<Val>() != comb.1 {
            error!(
                "Failed state for: {} Expected: {} got: {}",
                comb.0,
                comb.1,
                get_state::<Val>()
            );
            bail!("UI test failed")
        }
    }
    Ok(())
}

async fn inject_touch(touch: impl Into<Touch> + Send + Copy + 'static) {
    sleep(INJECT_TOUCH_DELAY);
    from_main(move || {
        App::current_mut().process_touch_event(touch.into());
    })
    .await;
}

pub async fn inject_touches(data: &str) {
    for touch in Touch::vec_from_str(data) {
        inject_touch(touch).await;
    }
}

pub async fn inject_keys(s: impl ToString) {
    let s = s.to_string();
    for ch in s.chars() {
        inject_key(ch).await
    }
}

pub async fn inject_key(key: char) {
    from_main(move || App::current_mut().on_char(key)).await
}

#[allow(dead_code)]
pub async fn record_touches() {
    record_touches_internal(true).await
}

#[allow(dead_code)]
pub async fn record_moved_touches() {
    record_touches_internal(false).await
}

async fn record_touches_internal(skip_moved: bool) {
    let touches = Vec::<Touch>::new().to_own();
    let mut touches = touches.weak();

    let (s, mut r) = channel::<()>(1);

    let moved_record_skip = 10;

    let moved_counter = Arc::new(Mutex::new(0));

    on_main(move || {
        UIEvents::on_touch().val(move |touch| {
            if touch.is_moved() {
                let mut counter = moved_counter.lock().unwrap();
                *counter += 1;
                if *counter == moved_record_skip {
                    *counter = 0;
                } else {
                    return;
                }
            }

            if skip_moved && touch.is_moved() {
                return;
            }

            touches.push(touch);
        });

        UIManager::keymap().add(UIManager::root_view(), 'a', move || {
            _ = s.try_send(());
        })
    });

    if let None = r.recv().await {
        warn!("Failed to receive record_touches result");
    }

    from_main(|| {
        UIEvents::on_touch().remove_subscribers();
    })
    .await;

    println!(
        r#"
        inject_touches(
        {}
{}
        {},
    )
        .await;
    "#,
        "r#\"",
        Touch::str_from_vec(touches.to_vec()),
        "\"#"
    );
}

#[allow(dead_code)]
pub async fn record_ui_test() -> Result<()> {
    loop {
        record_touches().await;
        record_touches_with_colors().await?;
    }
}

#[allow(dead_code)]
pub async fn record_touches_with_colors() -> Result<()> {
    let screenshot = App::take_screenshot().await?;

    let touches = Vec::<(Touch, U8Color)>::new().to_own();
    let mut touches = touches.weak();

    let (s, mut r) = channel::<()>(1);

    on_main(move || {
        UIEvents::on_touch().val(move |touch| {
            if !touch.is_began() {
                return;
            }

            touches.push((touch, screenshot.get_pixel(touch.position)));
        });

        UIManager::keymap().add(UIManager::root_view(), 'a', move || {
            _ = s.try_send(());
        })
    });

    if let None = r.recv().await {
        warn!("Failed to receive record_touches_with_colors result");
    }

    on_main(|| {
        UIEvents::on_touch().remove_subscribers();
    });

    println!("check_colors( r#\"");

    for (touch, color) in touches.deref() {
        let x: u32 = touch.position.x.lossy_convert();
        let y: u32 = touch.position.y.lossy_convert();
        println!(
            "            {:>4} {:>4} - {:>3} {:>3} {:>3}",
            x, y, color.r, color.g, color.b
        );
    }

    println!("        \"#");
    println!(").await?;");

    Ok(())
}

pub fn assert_eq<T: PartialEq<U> + Debug, U: Debug>(a: T, b: U) -> Result<()> {
    if a == b {
        return Ok(());
    }

    let message = format!("Assertion failed: {a:?} != {b:?}");
    error!("{message}");

    sleep(20.0);

    bail!(message)
}
