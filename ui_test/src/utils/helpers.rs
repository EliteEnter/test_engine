use anyhow::Result;
use test_engine::{
    gm::Apply,
    ui::{Button, Color, Container, Point, Screenshot, U8Color, UIManager, ViewData, ViewSubviews, WeakView},
    App,
};

pub fn add_corners(mut view: WeakView, color: Color) {
    let _1 = view.add_view::<Container>();
    let _2 = view.add_view::<Container>();
    let _3 = view.add_view::<Container>();
    let _4 = view.add_view::<Container>();

    [_1, _2, _3, _4].apply(|mut a| {
        a.place().size(100, 100);
        a.set_color(color);
    });

    _1.place().tl(0);
    _2.place().tr(0);
    _3.place().bl(0);
    _4.place().br(0);
}

#[allow(dead_code)]
pub fn add_action(action: impl FnMut() + 'static) {
    let mut button = UIManager::root_view().add_view::<Button>();
    button.place().size(100, 100).bl(0);
    button.set_color(Color::LIGHT_GRAY);
    button.on_tap(action);
    button.label = "Debug Action Button".into();
}

pub async fn check_colors(data: &str) -> Result<()> {
    let screenshot = App::take_screenshot().await?;

    let lines: Vec<_> = data.split("\n").collect();

    for line in lines {
        let parts: Vec<_> = line.split("-").collect();

        if parts.len() != 2 {
            continue;
        }

        let pos = parts[0];
        let color = parts[1];

        let pos: Vec<_> = pos.split(" ").filter(|a| !a.is_empty()).collect();
        let color: Vec<_> = color.split(" ").filter(|a| !a.is_empty()).collect();

        let pos: Point = Point::new(pos[0].parse().unwrap(), pos[1].parse().unwrap());
        let color: U8Color = U8Color::rgba(
            color[0].parse().unwrap(),
            color[1].parse().unwrap(),
            color[2].parse().unwrap(),
            255,
        );

        check_pixel_color(&screenshot, pos, color);
    }

    Ok(())
}

pub fn check_pixel_color(screenshot: &Screenshot, pos: Point, color: U8Color) {
    let pixel: U8Color = screenshot.get_pixel(pos);
    let pixel_f32: Color<f32> = pixel.into();
    let color_f32: Color<f32> = color.into();

    let diff = pixel_f32.diff(color_f32);

    let max_diff = 0.024;

    if diff > max_diff {
        panic!(
            "Color diff is too big: {diff}. Max: {max_diff}. Position: {pos:?}. \nExpected: {color}, got: \
             {pixel}"
        )
    }
}
