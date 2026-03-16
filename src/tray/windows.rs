
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, menu::Menu, Result,};
use image;

fn load_icon() -> Icon {
    let bytes = include_bytes!("icon16.ico");
    let image = image::load_from_memory(bytes)
    .unwrap().to_rgba8();
    let (width, height) = image.dimensions();
    Icon::from_rgba(image.into_raw(), width, height).unwrap()
}

pub fn build() -> Result<TrayIcon>{
    let icon = load_icon();
    let tray_menu = Menu::new();

    TrayIconBuilder::new()
    .with_icon(icon)
    .with_menu(Box::new(tray_menu))
    .build()
}