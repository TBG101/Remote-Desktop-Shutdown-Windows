use tray_icon::{
    menu::{Menu, MenuItem},
    Icon, TrayIconBuilder,
};

use gtk;
#[cfg(target_os = "linux")]
pub fn build_tray() {
    std::thread::spawn(|| {
        gtk::init().unwrap();

        let tray_menu = Menu::new();
        let quit_i = MenuItem::new("Quit", true, None);
        
        tray_menu.append(&quit_i).unwrap();

        let rgba: Vec<u8> = vec![
            255, 0, 0, 255, // Red pixel
            0, 255, 0, 255, // Green pixel
            0, 0, 255, 255, // Blue pixel
            255, 255, 0, 255, // Yellow pixel
        ];
        let icon = Icon::from_rgba(rgba, 2, 2).unwrap();

        let quit_i = MenuItem::with_id(0, "Quit", true, None);

        let tray_icon = TrayIconBuilder::new()
            .with_icon(icon)
            .with_menu(Box::new(tray_menu))
            .build()
            .unwrap();

        gtk::main();
    }).join().unwrap();
}
#[cfg(target_os = "windows")]
pub fn build_tray() {
    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("Quit", true, None);
    tray_menu.append(&quit_i).unwrap();

    let icon = Icon::from_file("icon.ico").unwrap();

    let quit_i = MenuItem::with_id(0, "Quit", true, None);

    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(tray_menu))
        .build()
        .unwrap();

    tray_icon.run();
}
