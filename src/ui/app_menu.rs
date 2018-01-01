//fn create_menu(window: &Window) -> MenuBar {
//    let menu_bar = MenuBar::new();
//    let about = MenuItem::new_with_label("About");
//    let window = window.to_owned();
//
//    about.connect_activate(move |_| {
//        let dialog = AboutDialog::new();
//        dialog.set_authors(&["@akavolkol"]);
//        dialog.set_title("About");
//        dialog.set_copyright(Some("2017"));
//        dialog.set_license(Some("MIT"));
//        dialog.set_transient_for(Some(&window));
//
//        dialog.run();
//        dialog.destroy();
//    });
//
//    menu_bar.append(&about);
//
//    menu_bar
//}