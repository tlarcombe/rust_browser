use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Statusbar, Orientation};
use webkit2gtk::{WebView, WebViewExt, WebContext, WebContextExt, CookieManagerExt, CookiePersistentStorage};
use gdk_pixbuf::Pixbuf;
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
    fn new(app: &Application, url: String, title: &str, data_dir: &str) -> Self {
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

        // Create persistent data directory for this instance
        let data_path = Path::new(data_dir);
        fs::create_dir_all(data_path).expect("Failed to create data directory");

        // Create a WebContext for this instance
        // Note: webkit2gtk automatically uses ~/.local/share/webkitgtk for default context
        // To have separate contexts per site, we use a unique context
        let context = WebContext::default().expect("Failed to create WebContext");

        // Configure persistent cookie storage
        let cookie_manager = context.cookie_manager().expect("Failed to get cookie manager");
        let cookie_file = format!("{}/cookies.sqlite", data_dir);
        cookie_manager.set_persistent_storage(&cookie_file, CookiePersistentStorage::Sqlite);

        // Create WebView with the configured context
        let web_view = WebView::with_context(&context);
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
        // Allow all navigation within the browser (no restrictions)
        // This enables OAuth flows and logging into websites

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
    // Try multiple config locations in order of preference
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_paths = vec![
        format!("{}/.config/trustbrowser/sites.conf", home),  // Installed location
        "sites.conf".to_string(),                              // Current directory
    ];

    for config_path in config_paths {
        if let Ok(contents) = fs::read_to_string(&config_path) {
            return contents
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
                .collect();
        }
    }

    // Default sites if no config file found
    vec![
        ("WhatsApp".to_string(), "https://web.whatsapp.com".to_string()),
        ("Google Calendar".to_string(), "https://calendar.google.com/calendar/u/0/r".to_string()),
        ("Messenger".to_string(), "https://www.messenger.com/e2ee/t/25417791761168110/".to_string()),
    ]
}

fn main() {
    // Load sites configuration
    let sites = load_config();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Handle --list flag
    if args.len() > 1 && (args[1] == "--list" || args[1] == "-l") {
        println!("Available sites:");
        for (i, (name, url)) in sites.iter().enumerate() {
            println!("  [{}] {} - {}", i, name, url);
        }
        std::process::exit(0);
    }

    // Determine site index from argument (number or name)
    let site_index: usize = if args.len() > 1 {
        let arg = &args[1];

        // Try to parse as number first
        if let Ok(index) = arg.parse::<usize>() {
            index
        } else {
            // Try to match by name (case-insensitive)
            let arg_lower = arg.to_lowercase();
            sites.iter()
                .position(|(name, _)| name.to_lowercase() == arg_lower)
                .unwrap_or_else(|| {
                    eprintln!("Error: Site '{}' not found. Available sites:", arg);
                    for (i, (name, url)) in sites.iter().enumerate() {
                        eprintln!("  [{}] {} - {}", i, name, url);
                    }
                    std::process::exit(1);
                })
        }
    } else {
        0  // Default to first site
    };

    // Validate site index
    if site_index >= sites.len() {
        eprintln!("Error: Site index {} is out of range. Available sites:", site_index);
        for (i, (name, url)) in sites.iter().enumerate() {
            eprintln!("  [{}] {} - {}", i, name, url);
        }
        std::process::exit(1);
    }

    let (site_name, site_url) = sites[site_index].clone();
    let window_title = format!("TRustBrowser - {}", site_name);

    // Create a unique data directory for each site to keep sessions separate
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let safe_site_name = site_name.to_lowercase().replace(' ', "_");
    let data_dir = format!("{}/.local/share/trustbrowser/{}", home, safe_site_name);

    // Use a unique application ID for each instance to allow multiple instances
    let app_id = format!("com.trustbrowser.app.instance{}", site_index);
    let app = Application::builder()
        .application_id(&app_id)
        .build();

    app.connect_activate(move |app| {
        let browser = Browser::new(app, site_url.clone(), &window_title, &data_dir);
        browser.setup_callbacks();
        browser.show();
    });

    // Use run_with_args(&[]) instead of run() to prevent GTK from trying to parse
    // command line arguments as files, which causes "can not open files" error
    app.run_with_args::<String>(&[]);
}
