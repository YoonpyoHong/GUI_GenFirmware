use aes::Aes256;
use cbc::cipher::block_padding::NoPadding;
use cbc::cipher::{BlockEncryptMut, KeyIvInit};
use p256::ecdsa::signature::Signer;
use p256::ecdsa::{Signature, SigningKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const KEY_SLOT_COUNT: usize = 10;
const HEADER_TOTAL_LEN: usize = 1024;
const HEADER_SIGNED_LEN: usize = 112;
const HEADER_SIGNATURE_LEN: usize = 64;
const HEADER_RESERVED_TAIL_LEN: usize = 848;
const BLOCK_ALIGN_LEN: usize = 2048;
const FINAL_SIGNATURE_LEN: usize = 64;

type Aes256CbcEnc = cbc::Encryptor<Aes256>;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    derived_key_index: Option<usize>,
    header_hash_prefix: Option<String>,
    key_slots: Vec<KeySlotCheck>,
    warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FirmwareGenerationResult {
    key_index: usize,
    header_path: String,
    encrypted_firmware_path: String,
    final_image_path: String,
    firmware_length: u32,
    encrypted_firmware_length: usize,
    final_image_length: usize,
    version_u32: u32,
    firmware_hash_hex: String,
    header_hash_hex: String,
    warnings: Vec<String>,
}

#[derive(Debug)]
struct HeaderBuildResult {
    header: Vec<u8>,
    header_unsigned: Vec<u8>,
    iv: [u8; 16],
    firmware_hash: [u8; 32],
}

#[tauri::command]
fn backend_status() -> String {
    "Rust backend is connected".to_string()
}

#[tauri::command]
fn validate_generation_inputs(request: FirmwareGenerationRequest) -> Result<FirmwareGenerationPlan, String> {
    build_generation_plan(request)
}

#[tauri::command]
fn generate_firmware_image(request: FirmwareGenerationRequest) -> Result<FirmwareGenerationResult, String> {
    let mut plan = build_generation_plan(request.clone())?;

    let firmware = fs::read(&request.firmware_path)
        .map_err(|error| format!("Failed to read firmware file: {error}"))?;

    if firmware.len() > u32::MAX as usize {
        return Err("Firmware file is too large for 4-byte length field".to_string());
    }

    fs::create_dir_all(&request.output_dir)
        .map_err(|error| format!("Failed to create output directory: {error}"))?;

    let version_u32 = build_version_u32(&request);
    let mut header_build = build_unsigned_header(version_u32, &firmware)?;
    let header_hash = Sha256::digest(&header_build.header_unsigned);
    let key_index = derive_key_index(&header_hash);

    let key_paths = key_paths(&request.key_root_dir, key_index);
    let aes_key = read_aes256_key(&key_paths.0)?;
    let signing_key = read_signing_key(&key_paths.1)?;

    let header_signature = sign_raw64(&signing_key, &header_build.header_unsigned);
    header_build.header.extend_from_slice(&header_signature);
    header_build.header.extend_from_slice(&vec![0u8; HEADER_RESERVED_TAIL_LEN]);

    if header_build.header.len() != HEADER_TOTAL_LEN {
        return Err(format!(
            "Internal header length error: expected {HEADER_TOTAL_LEN}, got {}",
            header_build.header.len()
        ));
    }

    let encrypted_firmware = encrypt_firmware_aes256_cbc_no_padding(
        &firmware,
        &aes_key,
        &header_build.iv,
    )?;

    let mut final_image = Vec::new();
    final_image.extend_from_slice(&header_build.header);
    final_image.extend_from_slice(&encrypted_firmware);

    let final_signature_offset = align_up(final_image.len() + FINAL_SIGNATURE_LEN, BLOCK_ALIGN_LEN) - FINAL_SIGNATURE_LEN;
    if final_image.len() < final_signature_offset {
        final_image.resize(final_signature_offset, 0u8);
    }

    let final_signature = sign_raw64(&signing_key, &final_image);
    final_image.extend_from_slice(&final_signature);

    if final_image.len() % BLOCK_ALIGN_LEN != 0 {
        return Err(format!(
            "Internal alignment error: final image length {} is not 2KB aligned",
            final_image.len()
        ));
    }

    let output_dir = Path::new(&request.output_dir);
    let header_path = output_dir.join("header.bin");
    let encrypted_firmware_path = output_dir.join("firmware.enc.bin");
    let final_image_path = output_dir.join("firmware_image.bin");

    fs::write(&header_path, &header_build.header)
        .map_err(|error| format!("Failed to write header.bin: {error}"))?;
    fs::write(&encrypted_firmware_path, &encrypted_firmware)
        .map_err(|error| format!("Failed to write firmware.enc.bin: {error}"))?;
    fs::write(&final_image_path, &final_image)
        .map_err(|error| format!("Failed to write firmware_image.bin: {error}"))?;

    plan.derived_key_index = Some(key_index);
    plan.header_hash_prefix = Some(hex_upper(&header_hash[..4]));

    Ok(FirmwareGenerationResult {
        key_index,
        header_path: header_path.display().to_string(),
        encrypted_firmware_path: encrypted_firmware_path.display().to_string(),
        final_image_path: final_image_path.display().to_string(),
        firmware_length: firmware.len() as u32,
        encrypted_firmware_length: encrypted_firmware.len(),
        final_image_length: final_image.len(),
        version_u32,
        firmware_hash_hex: hex_upper(&header_build.firmware_hash),
        header_hash_hex: hex_upper(&header_hash),
        warnings: plan.warnings,
    })
}

fn build_generation_plan(request: FirmwareGenerationRequest) -> Result<FirmwareGenerationPlan, String> {
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
            "Output directory will be created: {}",
            request.output_dir
        ));
    }

    let key_slots = (1..=KEY_SLOT_COUNT)
        .map(|index| {
            let (symmetric_key_path, private_key_path) = key_paths(&request.key_root_dir, index);
            let slot_dir = key_root.join(index.to_string());
            let exists = slot_dir.exists() && symmetric_key_path.exists() && private_key_path.exists();

            if !exists {
                warnings.push(format!(
                    "Key slot {} should contain {}.key and {}.pem",
                    index, index, index
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

    let version_u32 = build_version_u32(&request);
    let version_text = format!(
        "{}.{}.{}.{}",
        request.version_major, request.version_minor, request.version_patch, request.version_build
    );

    Ok(FirmwareGenerationPlan {
        firmware_path: request.firmware_path,
        output_dir: request.output_dir,
        version_u32,
        version_text,
        derived_key_index: None,
        header_hash_prefix: None,
        key_slots,
        warnings,
    })
}

fn build_unsigned_header(version_u32: u32, firmware: &[u8]) -> Result<HeaderBuildResult, String> {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);

    let firmware_hash_digest = Sha256::digest(firmware);
    let mut firmware_hash = [0u8; 32];
    firmware_hash.copy_from_slice(&firmware_hash_digest);

    let mut header = Vec::with_capacity(HEADER_TOTAL_LEN);
    header.extend_from_slice(b"FWAC");
    header.extend_from_slice(&version_u32.to_be_bytes());
    header.extend_from_slice(&(firmware.len() as u32).to_be_bytes());
    header.extend_from_slice(&0u32.to_be_bytes());
    header.extend_from_slice(&iv);
    header.extend_from_slice(&firmware_hash);
    header.extend_from_slice(&[0u8; 32]);
    header.push(0u8);
    header.extend_from_slice(&[0u8; 15]);

    if header.len() != HEADER_SIGNED_LEN {
        return Err(format!(
            "Internal unsigned header length error: expected {HEADER_SIGNED_LEN}, got {}",
            header.len()
        ));
    }

    Ok(HeaderBuildResult {
        header: header.clone(),
        header_unsigned: header,
        iv,
        firmware_hash,
    })
}

fn encrypt_firmware_aes256_cbc_no_padding(
    firmware: &[u8],
    key: &[u8; 32],
    iv: &[u8; 16],
) -> Result<Vec<u8>, String> {
    let mut padded = firmware.to_vec();
    let padded_len = align_up(padded.len(), 16);
    padded.resize(padded_len, 0u8);

    Ok(Aes256CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<NoPadding>(&padded))
}

fn read_aes256_key(path: &Path) -> Result<[u8; 32], String> {
    let key = fs::read(path).map_err(|error| {
        format!(
            "Failed to read AES key file {}: {error}",
            path.display()
        )
    })?;

    if key.len() != 32 {
        return Err(format!(
            "AES-256 key file must be exactly 32 bytes: {} has {} bytes",
            path.display(),
            key.len()
        ));
    }

    let mut result = [0u8; 32];
    result.copy_from_slice(&key);
    Ok(result)
}

fn read_signing_key(path: &Path) -> Result<SigningKey, String> {
    let pem = fs::read_to_string(path).map_err(|error| {
        format!(
            "Failed to read ECDSA private key PEM {}: {error}",
            path.display()
        )
    })?;

    SigningKey::from_pkcs8_pem(&pem)
        .map_err(|error| format!("Failed to parse ECDSA P-256 private key PEM: {error}"))
}

fn sign_raw64(signing_key: &SigningKey, data: &[u8]) -> [u8; 64] {
    let signature: Signature = signing_key.sign(data);
    signature.to_bytes().into()
}

fn derive_key_index(header_hash: &[u8]) -> usize {
    let value = u32::from_be_bytes([
        header_hash[0],
        header_hash[1],
        header_hash[2],
        header_hash[3],
    ]);
    (value as usize % KEY_SLOT_COUNT) + 1
}

fn key_paths(key_root_dir: &str, index: usize) -> (PathBuf, PathBuf) {
    let slot_dir = Path::new(key_root_dir).join(index.to_string());
    (
        slot_dir.join(format!("{index}.key")),
        slot_dir.join(format!("{index}.pem")),
    )
}

fn build_version_u32(request: &FirmwareGenerationRequest) -> u32 {
    ((request.version_major as u32) << 24)
        | ((request.version_minor as u32) << 16)
        | ((request.version_patch as u32) << 8)
        | (request.version_build as u32)
}

fn align_up(value: usize, alignment: usize) -> usize {
    if value == 0 {
        return 0;
    }

    ((value + alignment - 1) / alignment) * alignment
}

fn hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02X}")).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            backend_status,
            validate_generation_inputs,
            generate_firmware_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
