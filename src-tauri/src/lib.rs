// src-tauri/src/lib.rs --v2.3

use tauri::{
    menu::{Menu, MenuItem}, tray::TrayIconBuilder,
    Emitter, Manager, State,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable}; 
use serde_json::json;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::str::FromStr;
use std::sync::Mutex;

// 🌟 BỘ NHỚ LƯU TRỮ PHÍM TẮT ĐỘNG
struct AppShortcuts {
    text_shortcut: Mutex<Option<Shortcut>>,
    image_shortcut: Mutex<Option<Shortcut>>,
}

// src-tauri/src/lib.rs --v2.2 (Chỉ cập nhật 2 hàm API)

#[tauri::command]
async fn translate_text(api_key: String, text: String, target_lang: String, is_analyzer_mode: bool) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-lite:generateContent?key={}", api_key);
    
    // 🌟 RẼ NHÁNH PROMPT DỰA TRÊN CHẾ ĐỘ
    let prompt = if is_analyzer_mode {
        format!("You are an expert computer scientist, software engineer, and system administrator. Analyze the following terminal command, code snippet, or console output. Explain in detail what it does, identify any errors, bugs, or vulnerabilities if present, and provide actionable solutions or technical insights. Respond entirely in {}:\n\n{}", target_lang, text)
    } else {
        format!("You are a professional translator. Translate the following text into {}. Only return the translated text, no other comments:\n\n{}", target_lang, text)
    };

    let body = json!({ "contents": [{ "parts": [{"text": prompt}] }] });
    let res = client.post(&url).header("Content-Type", "application/json").json(&body).send().await.map_err(|e| format!("Network Error: {}", e))?;
    let json_res: serde_json::Value = res.json().await.map_err(|e| format!("Parse Error: {}", e))?;
    if let Some(error) = json_res.get("error") { return Err(format!("API Error: {}", error["message"].as_str().unwrap_or("Unknown"))); }
    if let Some(candidates) = json_res.get("candidates") {
        if let Some(content) = candidates[0]["content"]["parts"][0]["text"].as_str() { return Ok(content.to_string()); }
    }
    Err("Failed to read response from Gemini".to_string())
}

#[tauri::command]
async fn translate_image(api_key: String, base64_image: String, target_lang: String, is_analyzer_mode: bool) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);
    
    // 🌟 RẼ NHÁNH PROMPT DỰA TRÊN CHẾ ĐỘ
    let prompt = if is_analyzer_mode {
        format!("You are an expert computer scientist, software engineer, and system administrator. Extract the code, terminal commands, or console output from this image. Explain in detail what it does, identify any errors, bugs, or vulnerabilities if present, and provide actionable fixes or technical insights. Respond entirely in {}. Structure your response clearly, starting with the extracted text, followed by your technical analysis.", target_lang)
    } else {
        format!("Act as an expert OCR and translation system. Extract all text from this image and translate it into {}. Maintain the logical grouping of the text. For every line or distinct text block, present the result strictly in this exact format:\n[Original Text]\n↳ [Translated Text]\n\nDo not include any introductions, explanations, or extra formatting. Only return the bilingual pairs.", target_lang)
    };

    let body = json!({ "contents": [{ "parts": [{"text": prompt}, { "inline_data": { "mime_type": "image/png", "data": base64_image } }] }] });
    let res = client.post(&url).header("Content-Type", "application/json").json(&body).send().await.map_err(|e| format!("Network Error: {}", e))?;
    let json_res: serde_json::Value = res.json().await.map_err(|e| format!("Parse Error: {}", e))?;
    if let Some(error) = json_res.get("error") { return Err(format!("API Error: {}", error["message"].as_str().unwrap_or("Unknown"))); }
    if let Some(candidates) = json_res.get("candidates") {
        if let Some(content) = candidates[0]["content"]["parts"][0]["text"].as_str() { return Ok(content.to_string()); }
    }
    Err("Failed to read response from Gemini".to_string())
}

#[tauri::command]
fn get_clipboard_image_base64() -> Result<String, String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    let image = clipboard.get_image().map_err(|_| "Không tìm thấy ảnh. Nhấn Win+Shift+S trước nhé!".to_string())?;
    let img_buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(image.width as u32, image.height as u32, image.bytes.into_owned()).ok_or("Failed")?;
    let mut png_data = std::io::Cursor::new(Vec::new());
    img_buffer.write_to(&mut png_data, image::ImageFormat::Png).map_err(|e| e.to_string())?;
    let _ = clipboard.clear(); // Xóa RAM
    Ok(STANDARD.encode(png_data.into_inner()))
}

#[tauri::command]
fn hide_popup(handle: tauri::AppHandle) {
    if let Some(popup) = handle.get_webview_window("popup") { let _ = popup.hide(); }
}

// 🌟 HÀM XỬ LÝ DỊCH TEXT
fn handle_text_shortcut(handle: tauri::AppHandle) {
    let mut enigo = Enigo::new();
    // VÔ CÙNG QUAN TRỌNG: Nhả tất cả các phím modifier để tránh kẹt phím do người dùng đổi shortcut
    enigo.key_up(Key::Meta); enigo.key_up(Key::Shift); enigo.key_up(Key::Control); enigo.key_up(Key::Alt);
    enigo.key_down(Key::Control); enigo.key_click(Key::Layout('c')); enigo.key_up(Key::Control);

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let enigo_mouse = Enigo::new();
        let (mouse_x, mouse_y) = enigo_mouse.mouse_location();
        if let Some(popup) = handle.get_webview_window("popup") {
            let _ = popup.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: mouse_x + 15, y: mouse_y + 15 }));
            let _ = popup.show(); let _ = popup.set_focus();
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = handle.emit("trigger-translate-text", ());
    });
}

// 🌟 HÀM XỬ LÝ DỊCH ẢNH ĐA NỀN TẢNG (WINDOWS / LINUX)
fn handle_image_shortcut(handle: tauri::AppHandle) {
    let mut enigo = Enigo::new();
    // Nhả phím modifier chống kẹt
    enigo.key_up(Key::Meta); enigo.key_up(Key::Shift); enigo.key_up(Key::Control); enigo.key_up(Key::Alt);

    // Xóa clipboard cũ
    if let Ok(mut clipboard) = arboard::Clipboard::new() { let _ = clipboard.clear(); }

    // 🌟 PHÂN LUỒNG HỆ ĐIỀU HÀNH
    #[cfg(target_os = "windows")]
    {
        // Trên Windows: Gọi Snipping Tool mặc định
        enigo.key_down(Key::Meta); enigo.key_down(Key::Shift); enigo.key_click(Key::Layout('s')); enigo.key_up(Key::Shift); enigo.key_up(Key::Meta);
    }

    #[cfg(target_os = "linux")]
    {
        // Trên Linux: Dùng chuỗi lệnh bash tự động tìm công cụ chụp ảnh phù hợp
        // Lệnh sẽ thử chạy flameshot, nếu không có sẽ tự nhảy sang gnome-screenshot, spectacle...
        std::process::Command::new("sh")
            .arg("-c")
            .arg("flameshot gui || gnome-screenshot -a -c || spectacle -r -c -b || mate-screenshot -a -c || xfce4-screenshooter -r -c")
            .spawn()
            .expect("Lỗi: Không thể gọi bất kỳ công cụ chụp ảnh nào trên Linux");
    }

    // 🌟 Vòng lặp chờ Clipboard (Chạy chung cho cả 2 HDH)
    std::thread::spawn(move || {
        let mut image_captured = false;
        // Vòng lặp 30 lần (15 giây) chờ ảnh rơi vào Clipboard
        for _ in 0..30 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if clipboard.get_image().is_ok() { image_captured = true; break; }
            }
        }
        if image_captured {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let enigo_mouse = Enigo::new();
            let (mouse_x, mouse_y) = enigo_mouse.mouse_location();
            if let Some(popup) = handle.get_webview_window("popup") {
                let _ = popup.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: mouse_x + 15, y: mouse_y + 15 }));
                let _ = popup.show(); let _ = popup.set_focus();
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
            let _ = handle.emit("trigger-translate-image", ());
        } else {
            println!("⏳ Hết thời gian chờ chụp ảnh hoặc thao tác bị hủy.");
        }
    });
}

// 🌟 API ĐĂNG KÝ PHÍM TẮT ĐỘNG
#[tauri::command]
fn register_shortcuts(app: tauri::AppHandle, text_shortcut: String, image_shortcut: String, state: State<'_, AppShortcuts>) -> Result<String, String> {
    let global_shortcut = app.global_shortcut();

    // 1. Xóa phím tắt cũ
    if let Some(old_text) = state.text_shortcut.lock().unwrap().take() { let _ = global_shortcut.unregister(old_text); }
    if let Some(old_image) = state.image_shortcut.lock().unwrap().take() { let _ = global_shortcut.unregister(old_image); }

    // 2. Phân tích phím mới
    let new_text = Shortcut::from_str(&text_shortcut).map_err(|_| "Sai định dạng phím tắt Text".to_string())?;
    let new_image = Shortcut::from_str(&image_shortcut).map_err(|_| "Sai định dạng phím tắt Ảnh".to_string())?;

    // 3. Đăng ký lại hệ thống
    let app_handle_text = app.clone();
    global_shortcut.on_shortcut(new_text.clone(), move |_app, _shortcut, event| {
        if event.state == ShortcutState::Pressed { handle_text_shortcut(app_handle_text.clone()); }
    }).map_err(|e| format!("Lỗi đk phím Text: {}", e))?;

    let app_handle_image = app.clone();
    global_shortcut.on_shortcut(new_image.clone(), move |_app, _shortcut, event| {
        if event.state == ShortcutState::Pressed { handle_image_shortcut(app_handle_image.clone()); }
    }).map_err(|e| format!("Lỗi đk phím Ảnh: {}", e))?;

    // 4. Lưu vào biến State
    *state.text_shortcut.lock().unwrap() = Some(new_text);
    *state.image_shortcut.lock().unwrap() = Some(new_image);

    Ok("Đăng ký thành công!".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Kích hoạt biến State
        .manage(AppShortcuts {
            text_shortcut: Mutex::new(None),
            image_shortcut: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![translate_text, translate_image, get_clipboard_image_base64, hide_popup, register_shortcuts])
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init()) 
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))
        .setup(|app| {
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.hide();
                let main_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close(); let _ = main_clone.hide();
                    }
                });
            }

            let quit_item = MenuItem::with_id(app, "quit", "Quit App", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&settings_item, &quit_item])?;
            let _tray = TrayIconBuilder::new().icon(app.default_window_icon().unwrap().clone()).menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => std::process::exit(0),
                    "settings" => {
                        if let Some(main_window) = app.get_webview_window("main") {
                            let _ = main_window.show(); let _ = main_window.set_focus();
                        }
                    }
                    _ => {}
                }).build(app)?;

            // Bỏ trống khối lệnh shortcut ở đây vì JS sẽ lo việc đó!
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}