use image;
use tray_icon::{
    Icon, Result, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem, PredefinedMenuItem},
};

fn load_icon() -> Icon {
    let bytes = include_bytes!("icon16.ico");
    let image = image::load_from_memory(bytes).unwrap().to_rgba8();
    let (width, height) = image.dimensions();
    Icon::from_rgba(image.into_raw(), width, height).unwrap() //too lazy to properly check it rn
}

pub fn build_menu() -> tray_icon::menu::Result<Menu> {
    let logs = MenuItem::with_id("logs", "Open logs", true, None);
    let quit = MenuItem::with_id("quit", "Quit", true, None);

    let tray_menu = Menu::new();
    tray_menu.append(&logs)?;
    tray_menu.append(&PredefinedMenuItem::separator())?;
    tray_menu.append(&quit)?;
    Ok(tray_menu)
}

pub fn build_tray() -> Result<TrayIcon> {
    let icon = load_icon();
    let menu_items = match build_menu() {
        Ok(menu_items) => menu_items,
        Err(e) => {
            panic!("Tray icon build error: {}", e);
        }
    };

    TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu_items))
        .with_tooltip("esp32-sysmon")
        .build()
}
