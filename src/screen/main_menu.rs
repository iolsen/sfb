use imgui::*;

pub fn show<'a>(ui: &Ui<'a>) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
            MenuItem::new(im_str!("New Game"))
                .build(ui);
            menu.end(ui);
        }
        menu_bar.end(ui);
    }
}
