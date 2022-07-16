mod utils;
use gtk::prelude::*;
use gtk::AboutDialog;
use gtk::{gio, glib};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::env;
use crate::utils::sheets::*;

fn main() {
    utils::files::file_checker();
    utils::files::file_size();
    utils::sheets::make_document();
    let test = utils::sheets::read_sheet_val("Bobble Heads".to_string(), true);
    println!("{:?}", test);
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar_system"),
        Default::default(),
    );

    application.connect_startup(|app| {
        add_accelerators(app);
    });
    application.connect_activate(build_ui);

    application.run();
}

fn add_accelerators(application: &gtk::Application) {
    application.set_accels_for_action("app.about", &["F1"]);
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}

fn build_ui(application: &gtk::Application) {
    let windows: Rc<RefCell<HashMap<usize, glib::WeakRef<gtk::Window>>>> =
    Rc::new(RefCell::new(HashMap::new()));
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Fallout 76 Item Tracker");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let label = gtk::Label::new(Some("Nothing happened yet"));

    v_box.pack_start(&label, false, false, 0);
    window.add(&v_box);

    build_system_menu(application);

    add_actions(application, &label, &window);

    window.show_all();
}

fn build_system_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();
    let menu_bar = gio::Menu::new();
    let more_menu = gio::Menu::new();
    let settings_menu = gio::Menu::new();
    let submenu = gio::Menu::new();
    let sub2 = gio::Menu::new();

    // The first argument is the label of the menu item whereas the second is the action name. It'll
    // makes more sense when you'll be reading the "add_actions" function.
    menu.append(Some("Quit"), Some("app.quit"));


    submenu.append(Some("Bobble Heads"), Some("app.sub_sub_another2"));
    submenu.append(Some("Magazines"), Some("app.sub_sub_another"));
    submenu.append(Some("Appearel"), Some("app.sub_sub_another2"));
    sub2.append(Some("Trade"), Some("app.trade"));
    sub2.append(Some("Vendor"), Some("app.market"));
    settings_menu.append_submenu(Some("Items"), &submenu);
    menu_bar.append_submenu(Some("_Collectable List"), &settings_menu);
    menu_bar.append_submenu(Some("_Trade/Vendor"),&sub2);
    more_menu.append(Some("About"), Some("app.about"));
    menu_bar.append_submenu(Some("?"), &more_menu);

    application.set_app_menu(Some(&menu));
    application.set_menubar(Some(&menu_bar));
}

/// This function creates "actions" which connect on the declared actions from the menu items.
fn add_actions(
    application: &gtk::Application,
    label: &gtk::Label,
    window: &gtk::ApplicationWindow,
) {

    let sub_another = gio::SimpleAction::new("sub_another", None);
    sub_another.connect_activate(glib::clone!(@weak label => move |_, _| {
        label.set_text("sub another menu item clicked");
    }));
    let sub_sub_another = gio::SimpleAction::new("sub_sub_another", None);
    sub_sub_another.connect_activate(glib::clone!(@weak label => move |_, _| {
        label.set_text("sub sub another menu item clicked");
    }));
    let sub_sub_another2 = gio::SimpleAction::new("sub_sub_another2", None);
    sub_sub_another2.connect_activate(glib::clone!(@weak label => move |_, _| {
        label.set_text("sub sub another2 menu item clicked");
    }));
    let trade = gio::SimpleAction::new("trade", None);
    trade.connect_activate(glib::clone!(@weak label => move |_,_|{
        label.set_text("trade was clicked");
    }));
    let market = gio::SimpleAction::new("market", None);
    market.connect_activate(glib::clone!(@weak label => move |_,_|{
        label.set_text("market was clicked");
    }));

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(glib::clone!(@weak window => move |_, _| {
        let p = AboutDialog::new();
        p.set_website_label(Some("Web Site"));
        p.set_website(Some("https://thematrixservers.com"));
        p.set_authors(&["Sir Matrix"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.show_all();
    }));

    // We need to add all the actions to the application so they can be taken into account.
    application.add_action(&about);
    application.add_action(&quit);
    application.add_action(&sub_another);
    application.add_action(&sub_sub_another);
    application.add_action(&sub_sub_another2);
    application.add_action(&trade);
    application.add_action(&market);
}