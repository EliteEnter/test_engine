use crate::assets::Assets;
use crate::paths;
use crate::sprites::SpritesDrawer;
use crate::ui::ui_drawer::UIDrawer;
use crate::ui::{DebugView, TestView};
use gl_image::Image;
use gl_wrapper::{DesktopInput, GLWrapper, Screen};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, Key};
use gm::{Color, Point, Rect, Size};
use sprites::LevelBase;
use std::rc::Rc;
use tools::refs::{make_shared, new_shared, Shared};
use tools::New;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use ui::input::touch::{ButtonState, Event};
use ui::input::Touch;
use ui::{View, ViewBase};

pub struct TestScreen {
    cursor_position: Point,
    assets: Rc<Assets>,
    debug_view: Shared<DebugView>,
    root_view: Shared<dyn View>,
    level: Shared<LevelBase>,
    ui_drawer: UIDrawer,
    sprites_drawer: SpritesDrawer,
}

impl TestScreen {
    pub fn on_touch(&self, mut touch: Touch) {
        self.root_view.borrow_mut().check_touch(&mut touch);
        self.debug_view.borrow_mut().check_touch(&mut touch);
    }

    fn update_view(view: Shared<dyn View>) {
        let mut view = view.try_borrow_mut().unwrap();
        view.update();
        for view in view.subviews_mut() {
            TestScreen::update_view(view.clone());
        }
    }

    fn setup_level(&mut self) {
        let mut level = self.level.borrow_mut();

        level.setup();

        let square = Image::load(&paths::images().join("square.png"));

        level.add_sprite((0, 0, 1, 1).into());

        level
            .add_collider((0, 0, 100, 1).into())
            .borrow_mut()
            .set_image(square);

        level
            .add_collider((20, 0, 1, 100).into())
            .borrow_mut()
            .set_image(square);

        level
            .add_collider((-20, 0, 1, 100).into())
            .borrow_mut()
            .set_image(square);
        //
        // for i in 0..500 {
        //     level.add_rect((0.1 * i as f32, i * 2).into(), Size::square(0.5));
        // }
    }

    fn setup_test_view(&mut self) {
        let view = TestView::new();

        // let a = self.level.clone();
        // view.dpad.borrow_mut().on_up.subscribe(move |_| {
        //     a.borrow_mut().jump();
        // });
        //
        // let a = self.level.clone();
        // view.dpad.borrow_mut().on_left.subscribe(move |_| {
        //     a.borrow_mut().go_left();
        // });
        //
        // let a = self.level.clone();
        // view.dpad.borrow_mut().on_right.subscribe(move |_| {
        //     a.borrow_mut().go_right();
        // });
        //
        // let a = self.level.clone();
        // view.left_stick
        //     .borrow_mut()
        //     .on_direction_change
        //     .subscribe(move |direction| {
        //         a.borrow_mut().add_impulse(direction);
        //     });

        self.root_view.borrow_mut().add_subview(make_shared(view));
    }
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl DesktopInput for TestScreen {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::Moved,
        });
    }

    fn on_mouse_key_pressed(&self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(ButtonState::from_glfw(state)),
        })
    }

    fn on_key_pressed(&self, key: Key, action: Action) {
        self.level.borrow_mut().on_key_pressed(key, action)
    }
}

#[cfg(any(target_os = "ios", target_os = "android"))]
impl DesktopInput for TestScreen {}

impl Screen for TestScreen {
    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());

        self.debug_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());

        self.debug_view.borrow_mut().setup(self.debug_view.clone());

        self.setup_level();
        self.setup_test_view();
    }

    fn update(&mut self) {
        GLWrapper::clear();

        let mut level = self.level.borrow_mut();

        level.update();

        // self.sprites_drawer
        //     .set_camera_position(&level.player.borrow().position);

        for sprite in &level.sprites {
            self.sprites_drawer.draw(sprite);
        }

        //
        // for wall in &level.walls {
        //     let wall = wall.borrow();
        //     self.sprites_drawer.draw(&wall);
        // }

        TestScreen::update_view(self.root_view.clone());
        TestScreen::update_view(self.debug_view.clone());

        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw(self.root_view.clone());

        self.debug_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw(self.debug_view.clone());

        self.ui_drawer.reset_viewport();
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.borrow_mut().set_frame(Rect::from(size));
        self.sprites_drawer.set_resolution(&size);
        self.sprites_drawer.set_camera_position(&(0, 0).into());
        self.update();
    }
}

impl New for TestScreen {
    fn new() -> TestScreen {
        let mut font_path = ui::DEFAULT_FONT_PATH.lock().unwrap();
        *font_path = paths::fonts().join("SF.otf");
        drop(font_path);
        let assets = Assets::init();
        TestScreen {
            cursor_position: Point::new(),
            assets: assets.clone(),
            debug_view: new_shared::<DebugView>(),
            root_view: new_shared::<ViewBase>(),
            level: new_shared(),
            ui_drawer: UIDrawer::new(assets.clone()),
            sprites_drawer: SpritesDrawer::new(assets),
        }
    }
}
