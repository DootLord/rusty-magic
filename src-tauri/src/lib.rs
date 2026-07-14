use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::LazyLock;
use std::sync::Mutex;

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
static CARD_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

#[tauri::command]
async fn get_scryfall_url(name: &str) -> Result<String, String> {
    // Check Cache
    { // Lock the cache for reading, then drop the lock after checking ({ and } are used to limit the scope of the lock)})
        let cache = CARD_CACHE.lock().unwrap();
        if let Some(url) = cache.get(name) {
            println!("Cache hit for {name}: {url}");
            return Ok(url.clone());
        }
    }
    let scryfall_url: String = format!("https://api.scryfall.com/cards/named?fuzzy={name}");

    let client = reqwest::Client::builder()
        .user_agent("rusty-magic/0.1")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(scryfall_url)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Scryfall URL Get Error: {e}"))?;

    let card_json = match response.json::<ScryfallCard>().await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Error decoding Scryfall JSON: {e}")),
    };

    // Update Cache
    {
        let mut cache = CARD_CACHE.lock().unwrap();
        cache.insert(name.to_string(), card_json.image_uris.normal.to_string());
        println!("Cache updated for {name}: {}", card_json.image_uris.normal);
    }

    Ok(card_json.image_uris.normal.to_string())
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
            get_scryfall_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
