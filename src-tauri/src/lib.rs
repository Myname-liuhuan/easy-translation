mod commands;
mod language_detect;
mod translator;

use commands::translate::translate_text;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Position, PhysicalPosition,
};
use tauri_plugin_log::{Builder as LogBuilder, Target, TargetKind};

/// Tauri command to set activation policy (show/hide from Dock)
#[cfg(target_os = "macos")]
#[tauri::command]
fn set_dock_visibility(app: AppHandle, visible: bool) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSRunningApplication, NSApplicationActivationOptions};

    let policy = if visible {
        tauri::ActivationPolicy::Regular
    } else {
        tauri::ActivationPolicy::Accessory
    };
    let _ = app.set_activation_policy(policy);

    // Use objc2 API to properly update Dock icon
    let mtm = MainThreadMarker::new().expect("must be on main thread");
    let ns_app = NSApplication::sharedApplication(mtm);
    let cocoa_policy = if visible {
        NSApplicationActivationPolicy::Regular
    } else {
        NSApplicationActivationPolicy::Accessory
    };
    ns_app.setActivationPolicy(cocoa_policy);
    if visible {
        // Unhide and activate the app to refresh Dock icon
        let running_app = NSRunningApplication::currentApplication();
        let _ = running_app.unhide();
        running_app.activateWithOptions(NSApplicationActivationOptions::ActivateAllWindows);
    }
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
fn set_dock_visibility(_app: AppHandle, _visible: bool) {}

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
fn setup_macos_window_buttons(window: &tauri::WebviewWindow) {
    use objc2::runtime::AnyObject;
    use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};

    let ns_window_ptr = window.ns_window().unwrap();
    let ns_window = unsafe { &*(ns_window_ptr as *const AnyObject) };
    unsafe {
        // Enable fullscreen: FullScreenPrimary (1 << 7) | Managed (1 << 2)
        let window: &NSWindow = AnyObject::downcast_ref(ns_window).expect("not an NSWindow");
        let behavior = NSWindowCollectionBehavior::FullScreenPrimary
            | NSWindowCollectionBehavior::Managed;
        window.setCollectionBehavior(behavior);
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
        .invoke_handler(tauri::generate_handler![translate_text, set_dock_visibility]);

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
            // Set activation policy to Regular on macOS to show in Dock
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Regular);
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
                        let app_handle = tray.app_handle();
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = position_window_top_right(&window);
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                        // Show in Dock when window is shown
                        #[cfg(target_os = "macos")]
                        {
                            use objc2::MainThreadMarker;
                            use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSRunningApplication, NSApplicationActivationOptions, NSRequestUserAttentionType};
                            let mtm = MainThreadMarker::new().expect("must be on main thread");
                            let ns_app = NSApplication::sharedApplication(mtm);
                            // Set activation policy to regular first (show in Dock)
                            ns_app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
                            // Request user attention to force refresh Dock icon
                            ns_app.requestUserAttention(NSRequestUserAttentionType::InformationalRequest);
                            // Unhide and activate the app
                            let running_app = NSRunningApplication::currentApplication();
                            let _ = running_app.unhide();
                            running_app.activateWithOptions(NSApplicationActivationOptions::ActivateAllWindows);
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
                        // Hide the window first
                        if let Some(win) = app_handle.get_webview_window("main") {
                            let _ = win.hide();
                        }
                        // Hide from Dock when window is closed
                        #[cfg(target_os = "macos")]
                        {
                            use objc2::MainThreadMarker;
                            use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSRunningApplication};
                            let mtm = MainThreadMarker::new().expect("must be on main thread");
                            let ns_app = NSApplication::sharedApplication(mtm);
                            // Set activation policy to accessory (hide from Dock)
                            ns_app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
                            // Also call hide on the running app
                            let running_app = NSRunningApplication::currentApplication();
                            running_app.hide();
                        }
                        // Emit event to frontend to clear data
                        let _ = app_handle.emit("clear-data", ());
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
