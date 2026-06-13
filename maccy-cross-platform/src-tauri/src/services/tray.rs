use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    Manager, Runtime,
};

#[cfg(target_os = "macos")]
use cocoa::base::nil;
#[cfg(target_os = "macos")]
use cocoa::foundation::{NSRect, NSString};

/// Position the main window below the tray icon (macOS) or at top-center
pub fn position_window_near_tray<R: Runtime>(window: &tauri::WebviewWindow<R>) {
    let win_w = 380.0_f64;

    #[cfg(target_os = "macos")]
    let (x, y) = {
        if let Some(rect) = get_tray_icon_rect() {
            let screen_h = rect.size.height as f64;
            let tray_x = rect.origin.x as f64;
            let tray_y = rect.origin.y as f64;
            let tray_w = rect.size.width as f64;

            // Cocoa coords: origin at bottom-left. Convert to top-left origin.
            let tray_bottom_screen_y = screen_h - tray_y;
            let tray_center_x = tray_x + tray_w / 2.0;
            let gap = 8.0;

            let x = (tray_center_x - win_w / 2.0).max(4.0);
            let y = tray_bottom_screen_y + gap;
            (x, y)
        } else {
            fallback_position(win_w)
        }
    };

    #[cfg(not(target_os = "macos"))]
    let (x, y) = (100.0, 30.0);

    let _ = window.set_position(tauri::PhysicalPosition::new(x as i32, y as i32));
}

#[cfg(target_os = "macos")]
fn fallback_position(win_w: f64) -> (f64, f64) {
    use objc::{class, msg_send, sel, sel_impl};
    unsafe {
        let screen: *mut objc::runtime::Object = msg_send![class!(NSScreen), mainScreen];
        if !screen.is_null() {
            let frame: NSRect = msg_send![screen, frame];
            let x = ((frame.size.width as f64) - win_w) / 2.0;
            return (x.max(4.0), 28.0);
        }
    }
    (100.0, 30.0)
}

#[cfg(target_os = "macos")]
fn get_tray_icon_rect() -> Option<NSRect> {
    use objc::{class, msg_send, sel, sel_impl};

    unsafe {
        let pool: *mut objc::runtime::Object = msg_send![class!(NSAutoreleasePool), new];

        // Get status bar
        let status_bar: *mut objc::runtime::Object =
            msg_send![class!(NSStatusBar), systemStatusBar];
        if status_bar.is_null() {
            let _: () = msg_send![pool, release];
            return None;
        }

        // Get status items via KVC
        let key = NSString::alloc(nil).init_str("statusItems");
        let items: *mut objc::runtime::Object = msg_send![status_bar, valueForKey: key];
        if items.is_null() {
            let _: () = msg_send![pool, release];
            return None;
        }

        let count: usize = msg_send![items, count];
        if count == 0 {
            let _: () = msg_send![pool, release];
            return None;
        }

        let status_item: *mut objc::runtime::Object = msg_send![items, objectAtIndex: 0usize];
        let button: *mut objc::runtime::Object = msg_send![status_item, button];
        if button.is_null() {
            let _: () = msg_send![pool, release];
            return None;
        }

        let frame: NSRect = msg_send![button, frame];
        let window: *mut objc::runtime::Object = msg_send![button, window];
        if window.is_null() {
            let _: () = msg_send![pool, release];
            return None;
        }

        let screen_rect: NSRect = msg_send![window, convertRectToScreen: frame];
        let _: () = msg_send![pool, release];
        Some(screen_rect)
    }
}

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "显示 Maccy", true, None::<&str>)?;
    let clear = MenuItem::with_id(app, "clear", "清空历史", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &clear, &quit])?;

    tauri::tray::TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .menu(&menu)
        .tooltip("Maccy - 剪贴板管理器")
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    position_window_near_tray(&window);
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "clear" => {
                log::info!("Clear history from tray menu");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    match window.is_visible() {
                        Ok(true) => {
                            let _ = window.hide();
                        }
                        _ => {
                            position_window_near_tray(&window);
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
