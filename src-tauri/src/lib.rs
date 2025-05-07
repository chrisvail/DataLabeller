use mesh::io::{parse_mesh_file, ThreeMesh};

pub mod mesh;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_mesh_file])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_mesh_file(path: String) -> Result<ThreeMesh, String> {
    println!("Loading file: {}", path);

    parse_mesh_file(path).map(|mesh| mesh.into())
    // Ok(ThreeMesh::new(
    //     vec![0, 1, 2, 1, 3, 2],
    //     vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0],
    //     4,
    //     Some(vec![1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,1.0,1.0,1.0]),
    //     None,
    //     None,
    // ))
}
