use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::LazyLock;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CollectionSummary {
    unique_cards: u32,
    total_cards: u32,
}

// Scryfall URL response format
// All non-conforming response data is ignored! (Thankfully!!)
#[derive(Deserialize)]
struct ImageUris {
    normal: String,
}

#[derive(Deserialize)]
struct ScryfallCard {
    image_uris: ImageUris,
}

static COUNTER: AtomicI32 = AtomicI32::new(0);

// In memory URL cache for Scryfall card images. This is a simple HashMap wrapped in a Mutex for thread safety.
static CARD_URL_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// Need a request client with headers for Scryfall API Requests.
static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent("rusty-magic/0.1")
        .build()
        .expect("Failed to build reqwest client")
});

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn increment() -> i32 {
    return COUNTER.fetch_add(1, Ordering::SeqCst);
}

#[tauri::command]
fn get_current_value() -> i32 {
    return COUNTER.load(Ordering::SeqCst);
}

#[tauri::command]
fn get_collection_summary() -> CollectionSummary {
    CollectionSummary {
        unique_cards: 12,
        total_cards: 27,
    }
}

async fn get_scryfall_url(name: &str) -> Result<String, String> {
    let mut image_url: String = String::new();

    // Check URL Cache
    {
        // Lock the cache for reading, then drop the lock after checking ({ and } are used to limit the scope of the lock)})
        let cache = CARD_URL_CACHE.lock().unwrap();
        if let Some(url) = cache.get(name) {
            println!("Cache hit for {name}: {url}");
            image_url = url.clone();
        }
    }

    // Get URL from Scryfall if not in cache
    if image_url.is_empty() {
        let scryfall_url: String = format!("https://api.scryfall.com/cards/named?fuzzy={name}");

        let response = CLIENT
            .get(scryfall_url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Scryfall URL Get Error: {e}"))?;

        let card_json = match response.json::<ScryfallCard>().await {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Error decoding Scryfall JSON: {e}")),
        };

        image_url = card_json.image_uris.normal.to_string();

        // Update Cache
        {
            let mut cache = CARD_URL_CACHE.lock().unwrap();
            cache.insert(name.to_string(), image_url.clone());
            println!("Cache updated for {name}: {}", image_url);
        }
    }

    return Ok(image_url);
}

#[tauri::command]
async fn get_card_image(app: tauri::AppHandle, name: &str) -> Result<String, String> {
    // Get Cache Directory
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Error getting cache directory: {e}"))?;

    // Create Cache Directory if it doesn't exist
    std::fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("Error creating cache directory: {e}"))?;

    // Check to see if the image is already cached
    let file_path = cache_dir.join(format!("{name}.jpg"));
    if file_path.exists() {
        let bytes =
            std::fs::read(&file_path).map_err(|e| format!("Error reading cached image: {e}"))?;
        let encoded = STANDARD.encode(&bytes);
        return Ok(format!("data:image/jpeg;base64,{encoded}"));
    }

    let image_url = get_scryfall_url(name)
        .await
        .map_err(|e| format!("Error getting Scryfall URL: {e}"))?;

    let image_bytes = CLIENT
        .get(&image_url)
        .send()
        .await
        .map_err(|e| format!("Scryfall Image Get Error: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("Scryfall Image Bytes Error: {e}"))?;

    std::fs::write(cache_dir.join(format!("{name}.jpg")), &image_bytes)
        .map_err(|e| format!("Error writing image to cache: {e}"))?;

    let encoded = STANDARD.encode(&image_bytes);
    Ok(format!("data:image/jpeg;base64,{encoded}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            increment,
            get_current_value,
            get_collection_summary,
            get_card_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
