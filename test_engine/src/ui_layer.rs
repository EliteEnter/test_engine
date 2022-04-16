use std::{ops::DerefMut, rc::Rc};

#[cfg(not(any(target_os = "ios", target_os = "android")))]
use gl_wrapper::events::Events;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, Key};
use gm::flat::Point;
use rtools::{Boxed, Rglica, ToRglica};
use sprites::SpritesDrawer;
use ui::{
    input::touch::{ButtonState, TouchEvent},
    view_base::{add_view, ViewBase},
    Touch, View,
};

use crate::{assets::Assets, debug_view::DebugView, game_view::GameView, ui_drawer::UIDrawer};

pub struct UILayer {
    pub cursor_position: Point,
    pub root_view:       Box<dyn View>,
    pub debug_view:      Rglica<DebugView>,
    pub view:            Rglica<dyn GameView>,

    pub sprites_drawer: Rglica<dyn SpritesDrawer>,

    pub drawer: UIDrawer,

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub events: Rglica<Events>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f64,
}

impl UILayer {
    pub fn new(assets: Rc<Assets>, sprites_drawer: Rglica<dyn SpritesDrawer>) -> Box<Self> {
        Box::new(Self {
            cursor_position: Default::default(),
            root_view: ViewBase::boxed(),
            debug_view: Default::default(),
            view: Default::default(),
            sprites_drawer,
            drawer: UIDrawer::new(assets),
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            events: Default::default(),
            fps: Default::default(),
            prev_time: Default::default(),
            frame_time: Default::default(),
        })
    }
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        error!("{:?}", touch);
        self.cursor_position = touch.position;
        if !self.root_view.check_touch(&mut touch) {
            self.view.pass_touch_to_level(touch)
        }
    }

    pub fn set_view(&mut self, mut view: Box<dyn GameView>) {
        view.set_sprites_drawer(self.sprites_drawer);
        if self.view.is_ok() {
            self.view.remove_from_superview();
        }
        self.view = view.to_rglica();
        let ui = self.to_rglica();
        self.view.set_ui(ui);
        self.root_view.add_subview(view);
    }

    pub fn add_debug_view(&mut self) {
        self.debug_view = add_view(self.root_view.deref_mut())
    }
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl UILayer {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    TouchEvent::Moved,
        })
    }

    fn on_mouse_click(&mut self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    TouchEvent::from_state(ButtonState::from_glfw(state)),
        })
    }

    fn on_key_pressed(&mut self, key: Key, action: Action) {
        if action != Action::Press {
            return;
        }

        self.view
            .level_mut()
            .on_key_pressed(key.get_name().unwrap_or_else({
                || {
                    match key {
                        Key::Space => " ",
                        _ => "unknown",
                    }
                    .into()
                }
            }))
    }

    pub fn setup_events(&mut self) {
        self.events
            .on_key_pressed
            .subscribe(self.to_rglica(), move |a, mut this| {
                this.on_key_pressed(a.0, a.1)
            });

        self.events
            .on_mouse_click
            .subscribe(self.to_rglica(), move |a, mut this| {
                this.on_mouse_click(a.0, a.1)
            });

        self.events
            .on_cursor_moved
            .subscribe(self.to_rglica(), move |a, mut this| this.on_cursor_moved(a))
    }
}
