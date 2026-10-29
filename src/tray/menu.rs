use systray::Application;

pub fn build_menu(app: &mut Application) {
    let _ = app.set_icon_from_file("icon.ico");

    _ = app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    });
}
