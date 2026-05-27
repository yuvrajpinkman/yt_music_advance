use tauri::menu::{Menu, MenuItem, Submenu, CheckMenuItem};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri_plugin_notification::NotificationExt;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,    
};
use std::sync::Mutex;

#[cfg(target_os = "windows")]
fn set_app_user_model_id() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::iter::once;

    unsafe {
        let id: Vec<u16> = OsStr::new("com.yuvraj.Ytmusic")
            .encode_wide()
            .chain(once(0))
            .collect();
        windows_sys::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID(id.as_ptr());
    }
}

#[tauri::command]
fn notify_song(app: tauri::AppHandle, title: String, artist: String) {
    let state = app.state::<Arc<AtomicBool>>();
    if !state.load(Ordering::Relaxed) {
        return;
    }
    let body = format!("{} — {}", artist, title);
    let _ = app
        .notification()
        .builder()
        .title("Now Playing")
        .body(&body)
        .show();
}

struct NotifMenuItems {
    on: CheckMenuItem<tauri::Wry>,
    off: CheckMenuItem<tauri::Wry>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "windows")]
    set_app_user_model_id();

    let notifications_enabled = Arc::new(AtomicBool::new(true));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .manage(notifications_enabled)
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            win.eval("
                (function() {
                    let lastTitle = '';
                    let lastArtist = '';
                    function getSongInfo() {
                        const titleEl = document.querySelector('.title.style-scope.ytmusic-player-bar');
                        const artistEl = document.querySelector('.byline.style-scope.ytmusic-player-bar');
                        return {
                            title: titleEl ? titleEl.textContent.trim() : '',
                            artist: artistEl ? artistEl.textContent.trim() : ''
                        };
                    }
                    setInterval(function() {
                        try {
                            const info = getSongInfo();
                            if (info.title && info.artist && (info.title !== lastTitle || info.artist !== lastArtist)) {
                                lastTitle = info.title;
                                lastArtist = info.artist;
                                window.__TAURI__.core.invoke('notify_song', {
                                    title: info.title,
                                    artist: info.artist
                                }).catch(function(e) { console.warn('notify_song error:', e); });
                            }
                        } catch(e) {
                            console.warn('YTM polling error:', e);
                        }
                    }, 2000);
                })();
            ").unwrap();

            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let notif_on = CheckMenuItem::with_id(app, "notif_on", "On", true, true, None::<&str>)?;
            let notif_off = CheckMenuItem::with_id(app, "notif_off", "Off", true, false, None::<&str>)?;
            let notif_submenu = Submenu::with_id_and_items(app, "notifications", "Notifications", true, &[&notif_on, &notif_off])?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &notif_submenu, &quit_item])?;

            app.manage(Mutex::new(NotifMenuItems {
                on: notif_on.clone(),
                off: notif_off.clone(),
            }));

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("YouTube Music")
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "notif_on" => {
                        let state = app.state::<Arc<AtomicBool>>();
                        state.store(true, Ordering::Relaxed);
                        let items = app.state::<Mutex<NotifMenuItems>>();
                        let items = items.lock().unwrap();
                        let _ = items.on.set_checked(true);
                        let _ = items.off.set_checked(false);
                    }
                    "notif_off" => {
                        let state = app.state::<Arc<AtomicBool>>();
                        state.store(false, Ordering::Relaxed);
                        let items = app.state::<Mutex<NotifMenuItems>>();
                        let items = items.lock().unwrap();
                        let _ = items.on.set_checked(false);
                        let _ = items.off.set_checked(true);
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![notify_song])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}  
