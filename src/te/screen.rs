use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::sprites::Scene;
use crate::te::paths;
use crate::te::sprites::sprites_drawer::SpritesDrawer;
use crate::te::ui::DebugView;
use crate::te::{Assets, UIDrawer};
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::Font;
use crate::ui::ViewBase;
use lazy_static::lazy_static;
use std::rc::Rc;
use std::sync::Mutex;
use tools::has_new::new;
use tools::refs::{make_shared, new_shared, Shared};
use tools::HasNew;

lazy_static! {
    pub static ref DEFAULT_FONT: Mutex<Font> =
        Mutex::new(Font::new(&paths::fonts().join("SF.otf"), 48).unwrap());
}

pub struct Screen {
    cursor_position: Point,
    assets: Rc<Assets>,
    debug_view: Shared<DebugView>,
    root_view: Shared<dyn View>,
    scene: Scene,
    ui_drawer: UIDrawer,
    sprites_drawer: SpritesDrawer,
}

impl Screen {
    pub fn with_view(self, view: impl View + 'static) -> Self {
        self.root_view.borrow_mut().add_subview(make_shared(view));
        self
    }

    pub fn on_touch(&self, mut touch: Touch) {
        self.root_view.borrow().check_touch(&mut touch);
    }

    fn update_view(view: Shared<dyn View>) {
        let mut view = view.try_borrow_mut().unwrap();
        view.update();
        for view in view.subviews_mut() {
            Screen::update_view(view.clone());
        }
    }

    pub fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);
        self.root_view
            .borrow_mut()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());

        self.scene
            .add_collider(Point::new(), Size::make(100.0, 0.1));
        self.scene.add_ball(Point::make(0.1, 10), 0.5);
        self.scene.add_ball(Point::make(0.3, 11), 0.5);
        self.scene.add_ball(Point::make(0.4, 12), 0.5);
        self.scene.add_ball(Point::make(0.5, 13), 0.5);
        self.scene.add_ball(Point::make(0.6, 14), 0.5);
        self.scene.add_ball(Point::make(0.7, 15), 0.5);
        self.scene.add_ball(Point::make(0.8, 16), 0.5);
    }

    pub fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.borrow_mut().set_frame(Rect::from(size));
        self.sprites_drawer.set_size(&size);
        self.sprites_drawer.set_camera_position(&Point::make(0, 0));
        self.update();
    }

    pub fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    pub fn on_mouse_key_pressed(&self, _: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state),
        })
    }

    pub fn update(&mut self) {
        GLWrapper::clear();

        self.scene.update();

        for sprite in &self.scene.sprites {
            let sprite = sprite.borrow();
            self.sprites_drawer.draw(&sprite);
        }

        for wall in &self.scene.walls {
            let wall = wall.borrow();
            self.sprites_drawer.draw(&wall);
        }

        Screen::update_view(self.root_view.clone());

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
}

impl HasNew for Screen {
    fn new() -> Screen {
        let assets = Assets::init();
        Screen {
            cursor_position: Point::new(),
            assets: assets.clone(),
            debug_view: new_shared::<DebugView>(),
            root_view: new_shared::<ViewBase>(),
            scene: new(),
            ui_drawer: UIDrawer::new(assets.clone()),
            sprites_drawer: SpritesDrawer::new(assets),
        }
    }
}
