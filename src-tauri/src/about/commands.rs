#[tauri::command]
pub async fn open_about(handle: tauri::AppHandle) {
    match tauri::WindowBuilder::new(
        &handle,
        "about",
        tauri::WindowUrl::App("/src/pages/about.html".into()),
    )
    .inner_size(400f64, 600f64)
    .title("Catalyst - About")
    .center()
    .decorations(false)
    .resizable(false)
    .build() {
        Ok(_) => (),
        Err(error) => {
            match error {
                tauri::Error::WindowLabelAlreadyExists(_) => (),
                _ => println!("Window create failed!")
            }
        },
    }
}
