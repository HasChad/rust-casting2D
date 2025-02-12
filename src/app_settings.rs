use macroquad::prelude::*;

pub const WIDHT: i32 = 640;
pub const HEIGHT: i32 = 480;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RayCaster2D".into(),
        icon: None,
        window_width: WIDHT,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
