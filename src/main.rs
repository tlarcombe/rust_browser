use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Statusbar, Orientation};
use webkit2gtk::{WebView, WebViewExt, NavigationPolicyDecision, PolicyDecisionType, PolicyDecisionExt, NavigationPolicyDecisionExt, URIRequestExt};
use gdk_pixbuf::Pixbuf;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::env;

struct Browser {
    window: ApplicationWindow,
    web_view: WebView,
    status_bar: Statusbar,
    url: String,
}

impl Browser {
    fn new(app: &Application, url: String, title: &str) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(title)
            .default_width(800)
            .default_height(600)
            .build();

        window.set_position(gtk::WindowPosition::Center);
        window.set_resizable(true);

        // Set window icon - try multiple sizes, starting with the largest
        let icon_paths = [
            "icons/trustbrowser-256.png",
            "icons/trustbrowser-128.png",
            "icons/trustbrowser-64.png",
            "icons/trustbrowser-48.png",
        ];

        for icon_path in &icon_paths {
            if Path::new(icon_path).exists() {
                if let Ok(pixbuf) = Pixbuf::from_file(icon_path) {
                    window.set_icon(Some(&pixbuf));
                    break;
                }
            }
        }

        let main_box = GtkBox::new(Orientation::Vertical, 0);
        window.add(&main_box);

        let web_view = WebView::new();
        web_view.set_vexpand(true);
        web_view.set_hexpand(true);

        let status_bar = Statusbar::new();
        let status_context = status_bar.context_id("main");
        status_bar.push(status_context, "Ready");
        status_bar.show();

        main_box.pack_start(&web_view, true, true, 0);
        main_box.pack_start(&status_bar, false, false, 0);

        // Make sure all widgets are visible
        web_view.show();
        status_bar.show();
        main_box.show_all();

        Self {
            window,
            web_view,
            status_bar,
            url,
        }
    }

    fn setup_callbacks(&self) {
        let url = self.url.clone();

        // Set up navigation policy to keep links within the same domain
        self.web_view.connect_decide_policy(move |_webview, decision, decision_type| {
            if decision_type == PolicyDecisionType::NavigationAction {
                if let Some(nav_decision) = decision.dynamic_cast_ref::<NavigationPolicyDecision>() {
                    if let Some(request) = nav_decision.request() {
                        if let Some(uri) = request.uri() {
                            let uri_str = uri.as_str();

                            // Extract the base domain from the configured URL
                            let base_domain = url.split("://")
                                .nth(1)
                                .and_then(|s| s.split('/').next())
                                .unwrap_or("");

                            // Only allow navigation within the same domain
                            if uri_str.contains(base_domain) {
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
        let status_bar_clone = self.status_bar.clone();
        self.web_view.connect_load_changed(move |_, load_event| {
            let status_context = status_bar_clone.context_id("main");
            match load_event {
                webkit2gtk::LoadEvent::Started => {
                    status_bar_clone.push(status_context, "Loading...");
                }
                webkit2gtk::LoadEvent::Finished => {
                    if let Some(uri) = web_view_clone.uri() {
                        status_bar_clone.push(status_context, &format!("Loaded: {}", uri));
                    } else {
                        status_bar_clone.push(status_context, "Loaded");
                    }
                }
                _ => {}
            }
        });
    }

    fn show(&self) {
        self.window.present();
        // Load the configured URL
        self.web_view.load_uri(&self.url);
    }
}

// Load configuration from sites.conf file
fn load_config() -> Vec<(String, String)> {
    let config_path = "sites.conf";

    if let Ok(contents) = fs::read_to_string(config_path) {
        contents
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|line| {
                let parts: Vec<&str> = line.splitn(2, '|').collect();
                if parts.len() == 2 {
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    ("Unknown".to_string(), line.trim().to_string())
                }
            })
            .collect()
    } else {
        // Default sites if config file doesn't exist
        vec![
            ("WhatsApp".to_string(), "https://web.whatsapp.com".to_string()),
            ("Google Calendar".to_string(), "https://calendar.google.com/calendar/u/0/r".to_string()),
            ("Messenger".to_string(), "https://www.messenger.com/e2ee/t/25417791761168110/".to_string()),
        ]
    }
}

fn main() {
    // Load sites configuration
    let sites = load_config();

    // Get site index from command line argument (default to 0)
    let args: Vec<String> = env::args().collect();
    let site_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };

    // Validate site index
    if site_index >= sites.len() {
        eprintln!("Error: Site index {} is out of range. Available sites:", site_index);
        for (i, (name, url)) in sites.iter().enumerate() {
            eprintln!("  {}: {} - {}", i, name, url);
        }
        std::process::exit(1);
    }

    let (site_name, site_url) = sites[site_index].clone();
    let window_title = format!("TRustBrowser - {}", site_name);

    // Use a unique application ID for each instance to allow multiple instances
    let app_id = format!("com.trustbrowser.app.instance{}", site_index);
    let app = Application::builder()
        .application_id(&app_id)
        .build();

    app.connect_activate(move |app| {
        let browser = Browser::new(app, site_url.clone(), &window_title);
        browser.setup_callbacks();
        browser.show();
    });

    app.run();
}
