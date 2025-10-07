use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Entry, Statusbar, Orientation};
use webkit2gtk::{WebView, WebViewExt, NavigationPolicyDecision, PolicyDecisionType, PolicyDecisionExt, NavigationPolicyDecisionExt, URIRequestExt};
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;

struct Browser {
    window: ApplicationWindow,
    web_view: WebView,
    url_entry: Entry,
    status_bar: Statusbar,
    back_button: Button,
    forward_button: Button,
    history: Rc<RefCell<Vec<String>>>,
    current_index: Rc<RefCell<usize>>,
}

impl Browser {
    fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("TRustBrowser")
            .default_width(800)
            .default_height(600)
            .build();
        
        window.set_position(gtk::WindowPosition::Center);
        window.set_resizable(true);

        let main_box = GtkBox::new(Orientation::Vertical, 0);
        window.add(&main_box);

        let nav_box = GtkBox::new(Orientation::Horizontal, 5);
        nav_box.set_margin_top(5);
        nav_box.set_margin_bottom(5);
        nav_box.set_margin_start(5);
        nav_box.set_margin_end(5);

        let back_button = Button::with_label("←");
        back_button.set_tooltip_text(Some("Back (Alt+Left)"));
        back_button.set_sensitive(false);
        
        let forward_button = Button::with_label("→");
        forward_button.set_tooltip_text(Some("Forward (Alt+Right)"));
        forward_button.set_sensitive(false);

        let url_entry = Entry::new();
        url_entry.set_placeholder_text(Some("Enter URL here and press Enter"));
        url_entry.set_hexpand(true);
        
        // Add a button to focus the URL entry for testing
        let focus_button = Button::with_label("Focus URL");
        let url_entry_for_focus = url_entry.clone();
        focus_button.connect_clicked(move |_| {
            url_entry_for_focus.grab_focus();
        });

        nav_box.pack_start(&back_button, false, false, 0);
        nav_box.pack_start(&forward_button, false, false, 0);
        nav_box.pack_start(&url_entry, true, true, 0);
        nav_box.pack_start(&focus_button, false, false, 0);

        let web_view = WebView::new();
        web_view.set_vexpand(true);
        web_view.set_hexpand(true);

        let status_bar = Statusbar::new();
        let status_context = status_bar.context_id("main");
        status_bar.push(status_context, "Ready");
        status_bar.show();

        main_box.pack_start(&nav_box, false, false, 0);
        main_box.pack_start(&web_view, true, true, 0);
        main_box.pack_start(&status_bar, false, false, 0);
        
        // Make sure all widgets are visible
        nav_box.show_all();
        web_view.show();
        status_bar.show();
        main_box.show_all();

        let history = Rc::new(RefCell::new(Vec::new()));
        let current_index = Rc::new(RefCell::new(0));

        Self {
            window,
            web_view,
            url_entry,
            status_bar,
            back_button,
            forward_button,
            history,
            current_index,
        }
    }

    fn setup_callbacks(&self) {
        // Set up navigation policy to open links in external browser
        self.web_view.connect_decide_policy(|_webview, decision, decision_type| {
            if decision_type == PolicyDecisionType::NavigationAction {
                if let Some(nav_decision) = decision.dynamic_cast_ref::<NavigationPolicyDecision>() {
                    if let Some(request) = nav_decision.request() {
                        if let Some(uri) = request.uri() {
                            let uri_str = uri.as_str();

                            // Only allow the initial WhatsApp Web page and same-origin navigation
                            if uri_str.starts_with("https://web.whatsapp.com") {
                                decision.use_();
                                return true;
                            } else {
                                // Open external links in chromium
                                let _ = Command::new("/usr/bin/chromium")
                                    .arg(uri_str)
                                    .spawn();
                                decision.ignore();
                                return true;
                            }
                        }
                    }
                }
            }
            false
        });

        let web_view_clone = self.web_view.clone();
        let history_clone = self.history.clone();
        let current_index_clone = self.current_index.clone();
        let back_button_clone = self.back_button.clone();
        let forward_button_clone = self.forward_button.clone();

        self.url_entry.connect_activate(move |entry| {
            let url = entry.text().to_string();
            if !url.is_empty() {
                let formatted_url = if !url.starts_with("http://") && !url.starts_with("https://") {
                    format!("https://{}", url)
                } else {
                    url.clone()
                };
                
                web_view_clone.load_uri(&formatted_url);
                
                let mut history = history_clone.borrow_mut();
                let mut index = current_index_clone.borrow_mut();
                
                history.truncate(*index + 1);
                history.push(formatted_url);
                *index = history.len() - 1;
                
                back_button_clone.set_sensitive(*index > 0);
                forward_button_clone.set_sensitive(false);
            }
        });

        let web_view_clone2 = self.web_view.clone();
        let status_bar_clone2 = self.status_bar.clone();
        self.web_view.connect_load_changed(move |_, load_event| {
            let status_context = status_bar_clone2.context_id("main");
            match load_event {
                webkit2gtk::LoadEvent::Started => {
                    status_bar_clone2.push(status_context, "Loading...");
                }
                webkit2gtk::LoadEvent::Finished => {
                    if let Some(uri) = web_view_clone2.uri() {
                        status_bar_clone2.push(status_context, &format!("Loaded: {}", uri));
                    } else {
                        status_bar_clone2.push(status_context, "Loaded");
                    }
                }
                _ => {}
            }
        });

        let history_clone2 = self.history.clone();
        let current_index_clone2 = self.current_index.clone();
        let web_view_clone3 = self.web_view.clone();
        let back_button_clone2 = self.back_button.clone();
        let forward_button_clone2 = self.forward_button.clone();
        let url_entry_clone2 = self.url_entry.clone();

        self.back_button.connect_clicked(move |_| {
            let mut index = current_index_clone2.borrow_mut();
            let history = history_clone2.borrow();
            
            if *index > 0 {
                *index -= 1;
                if let Some(url) = history.get(*index) {
                    web_view_clone3.load_uri(url);
                    url_entry_clone2.set_text(url);
                }
                
                back_button_clone2.set_sensitive(*index > 0);
                forward_button_clone2.set_sensitive(*index < history.len() - 1);
            }
        });

        let history_clone3 = self.history.clone();
        let current_index_clone3 = self.current_index.clone();
        let web_view_clone4 = self.web_view.clone();
        let back_button_clone3 = self.back_button.clone();
        let forward_button_clone3 = self.forward_button.clone();
        let url_entry_clone3 = self.url_entry.clone();

        self.forward_button.connect_clicked(move |_| {
            let mut index = current_index_clone3.borrow_mut();
            let history = history_clone3.borrow();
            
            if *index < history.len() - 1 {
                *index += 1;
                if let Some(url) = history.get(*index) {
                    web_view_clone4.load_uri(url);
                    url_entry_clone3.set_text(url);
                }
                
                back_button_clone3.set_sensitive(*index > 0);
                forward_button_clone3.set_sensitive(*index < history.len() - 1);
            }
        });

        let url_entry_clone4 = self.url_entry.clone();
        let history_clone4 = self.history.clone();
        let current_index_clone4 = self.current_index.clone();
        let web_view_clone5 = self.web_view.clone();
        let back_button_clone4 = self.back_button.clone();
        let forward_button_clone4 = self.forward_button.clone();
        
        // Set up key event handling after the window is shown
        self.window.connect_realize(move |window| {
            // Enable key press events
            window.set_can_focus(true);
            window.grab_focus();
        });
        
        self.window.connect_key_press_event(move |_window, event| {
            let keyval = event.keyval();
            let state = event.state();
            
            if state.contains(gdk::ModifierType::CONTROL_MASK) && keyval == gdk::keys::constants::o {
                url_entry_clone4.grab_focus();
                url_entry_clone4.select_region(0, -1);
                return true.into();
            } else if state.contains(gdk::ModifierType::MOD1_MASK) && keyval == gdk::keys::constants::Left {
                let mut index = current_index_clone4.borrow_mut();
                let history = history_clone4.borrow();
                
                if *index > 0 {
                    *index -= 1;
                    if let Some(url) = history.get(*index) {
                        web_view_clone5.load_uri(url);
                        url_entry_clone4.set_text(url);
                    }
                    
                    back_button_clone4.set_sensitive(*index > 0);
                    forward_button_clone4.set_sensitive(*index < history.len() - 1);
                }
                return true.into();
            } else if state.contains(gdk::ModifierType::MOD1_MASK) && keyval == gdk::keys::constants::Right {
                let mut index = current_index_clone4.borrow_mut();
                let history = history_clone4.borrow();
                
                if *index < history.len() - 1 {
                    *index += 1;
                    if let Some(url) = history.get(*index) {
                        web_view_clone5.load_uri(url);
                        url_entry_clone4.set_text(url);
                    }
                    
                    back_button_clone4.set_sensitive(*index > 0);
                    forward_button_clone4.set_sensitive(*index < history.len() - 1);
                }
                return true.into();
            }
            
            false.into()
        });
    }

    fn show(&self) {
        self.window.present();

        // Load WhatsApp Web on launch
        self.web_view.load_uri("https://web.whatsapp.com");
        self.url_entry.set_text("https://web.whatsapp.com");

        // Add to history
        let mut history = self.history.borrow_mut();
        let mut index = self.current_index.borrow_mut();
        history.push("https://web.whatsapp.com".to_string());
        *index = 0;
    }
}

fn main() {
    let app = Application::builder()
        .application_id("com.trustbrowser.app")
        .build();

    app.connect_activate(|app| {
        let browser = Browser::new(app);
        browser.setup_callbacks();
        browser.show();
    });

    app.run();
}
