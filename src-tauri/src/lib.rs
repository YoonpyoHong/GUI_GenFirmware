use serde::{Deserialize, Serialize};
use std::path::Path;

const KEY_SLOT_COUNT: usize = 10;

#[derive(Debug, Deserialize)]\#[serde(rename_all = "camelCase")]
struct FirmwareGenerationRequest {
    key_root_dir: String,
    firmware_path: String,
    output_dir: String,
    version_major: u16,
    version_minor: u16,
    version_patch: u16,
    version_build: u16,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KeySlotCheck {
    index: usize,
    folder: String,
    symmetric_key_path: String,
    private_key_path: String,
    exists: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FirmwareGenerationPlan {
    firmware_path: String,
    output_dir: String,
    version_u32: u32,
    version_text: String,
    key_slots: Vec<KeySlotCheck>,
    warnings: Vec<String>,
}

#[tauri::command]
fn backend_status() -> String {
    "Rust backend is connected".to_string()
}

#[tauri::command]
fn validate_generation_inputs(request: FirmwareGenerationRequest) -> Result<FirmwareGenerationPlan, String> {
    let key_root = Path::new(&request.key_root_dir);
    let firmware_path = Path::new(&request.firmware_path);
    let output_dir = Path::new(&request.output_dir);

    if request.key_root_dir.trim().is_empty() {
        return Err("Key root directory is required".to_string());
    }

    if request.firmware_path.trim().is_empty() {
        return Err("Firmware file path is required".to_string());
    }

    if request.output_dir.trim().is_empty() {
        return Err("Output directory is required".to_string());
    }

    let mut warnings = Vec::new();

    if !key_root.exists() {
        warnings.push(format!(
            "Key root directory does not exist yet: {}",
            request.key_root_dir
        ));
    }

    if !firmware_path.exists() {
        warnings.push(format!(
            "Firmware file does not exist yet: {}",
            request.firmware_path
        ));
    }

    if !output_dir.exists() {
        warnings.push(format!(
            "Output directory does not exist yet: {}",
            request.output_dir
        ));
    }

    let key_slots = (0..KEY_SLOT_COUNT)
        .map(|index| {
            let slot_dir = key_root.join(index.to_string());
            let symmetric_key_path = slot_dir.join("symmetric.key");
            let private_key_path = slot_dir.join("private.pem");
            let exists = slot_dir.exists() && symmetric_key_path.exists() && private_key_path.exists();

            if !exists {
                warnings.push(format!(
                    "Key slot {} should contain symmetric.key and private.pem",
                    index
                ));
            }

            KeySlotCheck {
                index,
                folder: slot_dir.display().to_string(),
                symmetric_key_path: symmetric_key_path.display().to_string(),
                private_key_path: private_key_path.display().to_string(),
                exists,
            }
        })
        .collect::<Vec<_>>();

    let version_u32 = ((request.version_major as u32) << 24)
        | ((request.version_minor as u32) << 16)
        | ((request.version_patch as u32) << 8)
        | (request.version_build as u32);

    let version_text = format!(
        "{}.{}.{}.{}",
        request.version_major, request.version_minor, request.version_patch, request.version_build
    );

    Ok(FirmwareGenerationPlan {
        firmware_path: request.firmware_path,
        output_dir: request.output_dir,
        version_u32,
        version_text,
        key_slots,
        warnings,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            backend_status,
            validate_generation_inputs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
