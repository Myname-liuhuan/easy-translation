mod commands;
mod language_detect;
mod translator;

use commands::translate::translate_text;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Position, PhysicalPosition,
};
use tauri_plugin_log::{Builder as LogBuilder, Target, TargetKind};

/// Position window at top-right corner with 100px padding
fn position_window_top_right(window: &tauri::WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(monitor) = window.primary_monitor()? {
        let monitor_size = monitor.size();
        let monitor_position = monitor.position();
        let window_size = window.outer_size()?;

        let position = PhysicalPosition {
            x: monitor_position.x + monitor_size.width as i32 - window_size.width as i32 - 100,
            y: monitor_position.y + 100,
        };
        window.set_position(Position::Physical(position))?;
    }
    Ok(())
}

/// Setup macOS window buttons (fullscreen instead of zoom)
#[cfg(target_os = "macos")]
#[allow(deprecated)]
fn setup_macos_window_buttons(window: &tauri::WebviewWindow) {
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;

    let ns_window = window.ns_window().unwrap() as id;
    unsafe {
        // Enable fullscreen: NSWindowCollectionBehaviorFullScreenPrimary (1 << 7)
        //              | NSWindowCollectionBehaviorManaged (1 << 2)
        let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenPrimary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorManaged;
        ns_window.setCollectionBehavior_(behavior);
    }
}

#[cfg(not(target_os = "macos"))]
fn setup_macos_window_buttons(_window: &tauri::WebviewWindow) {}

#[cfg_attr(mobile, tauri::mobile_entrypoint)]
pub fn run() {
    // Initialize translation module (load .env file)
    translator::init();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![translate_text]);

    // Only enable logging in debug builds
    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(
            LogBuilder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        );
    }

    builder
        .setup(|app| {
            // Set activation policy to Accessory on macOS to hide from Dock
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Position main window at top-right and setup buttons
            if let Some(window) = app.get_webview_window("main") {
                let _ = position_window_top_right(&window);
                setup_macos_window_buttons(&window);
            }

            // Create quit menu item
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            // Create tray menu
            let menu = Menu::with_items(app, &[&quit_item])?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false) // Only show menu on right click
                .on_menu_event(|app: &AppHandle, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event: TrayIconEvent| {
                    // Show window on left click
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = position_window_top_right(&window);
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Listen for window close event and hide instead of closing
            let app_handle = app.app_handle().clone();
            if let Some(window) = app.get_webview_window("main") {
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent the window from closing
                        api.prevent_close();
                        // Hide the window instead
                        if let Some(win) = app_handle.get_webview_window("main") {
                            let _ = win.hide();
                        }
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
