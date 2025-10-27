use rusqlite::{params_from_iter, params, Connection, Result};
use std::fs;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[tauri::command]
fn get_tables() -> Result<Vec<std::collections::HashMap<String, String>>, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    // Create table_images if not exists
    conn.execute("CREATE TABLE IF NOT EXISTS table_images (table_name TEXT PRIMARY KEY, image TEXT);", []).map_err(|e| e.to_string())?;
    // Create settings table if not exists
    conn.execute("CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);", []).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name != 'table_images' AND name != 'settings';").map_err(|e| e.to_string())?;
    let table_names: Vec<String> = stmt.query_map([], |row| row.get(0)).map_err(|e| e.to_string())?.map(|r| r.unwrap()).collect();
    let mut tables = Vec::new();
    for name in table_names {
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), name.clone());
        // Get image
        let image: Option<String> = conn.query_row("SELECT image FROM table_images WHERE table_name = ?", [name], |row| row.get(0)).unwrap_or(None);
        map.insert("image".to_string(), image.unwrap_or_default());
        tables.push(map);
    }
    Ok(tables)
}

#[tauri::command]
fn get_records(tableName: String) -> Result<Vec<std::collections::HashMap<String, String>>, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    // Get column names
    let mut stmt = conn.prepare(&format!("PRAGMA table_info(`{}`);", tableName)).map_err(|e| e.to_string())?;
    let columns: Vec<String> = stmt.query_map([], |row| Ok(row.get::<_, String>(1)?)).map_err(|e| e.to_string())?.collect::<Result<_, _>>().map_err(|e| e.to_string())?;
    
    // Check if order_index column exists
    let has_order_index = columns.contains(&"order_index".to_string());
    let order_clause = if has_order_index { "ORDER BY order_index ASC, id ASC" } else { "ORDER BY id ASC" };
    
    let select_sql = format!("SELECT {} FROM `{}` {};", columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", "), tableName, order_clause);
    let mut stmt = conn.prepare(&select_sql).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        let mut map = std::collections::HashMap::new();
        for (i, col) in columns.iter().enumerate() {
            // Ensure id is read as integer and converted to string so frontend gets a valid id
            let val: String = if col == "id" {
                // attempt to read as i64, fall back to string
                match row.get::<_, i64>(i) {
                    Ok(n) => n.to_string(),
                    Err(_) => row.get::<_, String>(i).unwrap_or_else(|_| "".to_string()),
                }
            } else {
                row.get::<_, String>(i).unwrap_or_else(|_| "".to_string())
            };
            if !map.contains_key(col) {
                map.insert(col.clone(), val);
            }
        }
        Ok(map)
    }).map_err(|e| e.to_string())?;
    let mut records = Vec::new();
    for row in rows {
        records.push(row.map_err(|e| e.to_string())?);
    }
    Ok(records)
}

#[tauri::command]
fn set_table_image(tableName: String, imagePath: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    // Create table if not exists
    conn.execute("CREATE TABLE IF NOT EXISTS table_images (table_name TEXT PRIMARY KEY, image TEXT);", []).map_err(|e| e.to_string())?;
    // Read image file
    let image_data = fs::read(&imagePath).map_err(|e| e.to_string())?;
    let base64 = general_purpose::STANDARD.encode(&image_data);
    // Insert or update
    conn.execute("INSERT OR REPLACE INTO table_images (table_name, image) VALUES (?, ?)", params![tableName, base64]).map_err(|e| e.to_string())?;
    Ok("Immagine impostata.".to_string())
}

#[tauri::command]
fn delete_table_image(tableName: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM table_images WHERE table_name = ?", [tableName]).map_err(|e| e.to_string())?;
    Ok("Immagine eliminata.".to_string())
}

#[tauri::command]
fn update_record(tableName: String, id: String, updates: std::collections::HashMap<String, String>) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    // parse id
    let id_num: i64 = id.parse::<i64>().map_err(|e: std::num::ParseIntError| e.to_string())?;

    // If desc is being updated, ensure uniqueness (ignore current record)
    if let Some(new_desc) = updates.get("desc") {
        let count: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM `{}` WHERE `desc` = ? AND id != ?", tableName), params![new_desc, id_num], |row| row.get(0)).map_err(|e| e.to_string())?;
        if count > 0 {
            return Err("table.duplicate_desc".to_string());
        }
    }

    let set_clause = updates.keys().map(|k| format!("`{}` = ?", k)).collect::<Vec<_>>().join(", ");
    let sql = format!("UPDATE `{}` SET {} WHERE id = ?", tableName, set_clause);
    let params: Vec<String> = updates.values().cloned().collect();
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let mut param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
    param_refs.push(&id_num);
    stmt.execute(param_refs.as_slice()).map_err(|e| e.to_string())?;
    Ok("Record aggiornato.".to_string())
}

#[tauri::command]
fn delete_record(tableName: String, id: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    // parse id to integer
    let id_num: i64 = id.parse::<i64>().map_err(|e: std::num::ParseIntError| e.to_string())?;
    conn.execute(&format!("DELETE FROM `{}` WHERE id = ?", tableName), params![id_num]).map_err(|e| e.to_string())?;
    Ok("Record eliminato.".to_string())
}

#[tauri::command]
fn insert_record(tableName: String, record: std::collections::HashMap<String, String>) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    // Check if desc already exists
    if let Some(desc) = record.get("desc") {
        let count: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM `{}` WHERE `desc` = ?", tableName), [desc], |row| row.get(0)).map_err(|e| e.to_string())?;
        if count > 0 {
            return Err("table.duplicate_desc".to_string());
        }
    }
    let columns: Vec<String> = record.keys().cloned().collect();
    let placeholders = vec!["?".to_string(); columns.len()].join(", ");
    let quoted_columns = columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
    let sql = format!("INSERT INTO `{}` ({}) VALUES ({})", tableName, quoted_columns, placeholders);
    let values: Vec<String> = record.values().cloned().collect();
    let params = params_from_iter(values.iter());
    conn.execute(&sql, params).map_err(|e| e.to_string())?;
    Ok("Record inserito.".to_string())
}

#[tauri::command]
fn get_table_columns(tableName: String) -> Result<Vec<String>, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(&format!("PRAGMA table_info(`{}`);", tableName)).map_err(|e| e.to_string())?;
    let columns: Vec<String> = stmt.query_map([], |row| Ok(row.get::<_, String>(1)?)).map_err(|e| e.to_string())?.collect::<Result<_, _>>().map_err(|e| e.to_string())?;
    Ok(columns)
}

#[tauri::command]
async fn fetch_and_set_logo(tableName: String, gameName: String) -> Result<String, String> {
    let service = get_setting("api_service".to_string()).unwrap_or("thegamesdb".to_string());
    let api_key = get_setting("thegamesdb_api_key".to_string())?;
    if api_key.is_empty() {
        return Err("settings.api_key_missing".to_string());
    }

    let client = Client::new();

    if service == "rawg" {
        // RAWG API
        let search_url = format!("https://api.rawg.io/api/games?key={}&search={}&page_size=1", api_key, urlencoding::encode(&gameName));
        println!("fetch_and_set_logo: searching RAWG for '{}'", gameName);
        let response: RawgResponse = client.get(&search_url).send().await.map_err(|e| format!("Errore nella ricerca RAWG: {}", e))?.json().await.map_err(|e| format!("Errore nel parsing RAWG: {}", e))?;

        if response.results.is_empty() {
            return Err("home.no_game_found_rawg".to_string());
        }

        let game = &response.results[0];
    let image_url = game.background_image.as_ref().ok_or("home.no_image_available_rawg".to_string())?;
        println!("fetch_and_set_logo: downloading from RAWG {}", image_url);

        let response = client.get(image_url).send().await.map_err(|e| format!("Errore nel download RAWG: {}", e))?;
        let bytes = response.bytes().await.map_err(|e| format!("Errore nella lettura bytes RAWG: {}", e))?;
        println!("fetch_and_set_logo: downloaded {} bytes from RAWG", bytes.len());

        let temp_path = format!("../data/temp_logo_{}.png", tableName);
        fs::write(&temp_path, &bytes).map_err(|e| format!("Errore scrittura temp RAWG: {}", e))?;
        println!("fetch_and_set_logo: wrote temp file {}", temp_path);

        set_table_image(tableName, temp_path)
    } else {
        // TheGamesDB
        let search_url = format!("https://api.thegamesdb.net/v1/Games/ByGameName?apikey={}&name={}&fields=artworks", api_key, urlencoding::encode(&gameName));
        println!("fetch_and_set_logo: searching TheGamesDB for '{}'", gameName);
        let search_response: GamesResponse = client.get(&search_url).send().await.map_err(|e| format!("Errore nella ricerca TheGamesDB: {}", e))?.json().await.map_err(|e| format!("Errore nel parsing TheGamesDB: {}", e))?;

        if search_response.data.games.is_empty() {
            return Err("home.no_game_found_thegamesdb".to_string());
        }

        let game = &search_response.data.games[0];
        let game_id = game.id;
        println!("fetch_and_set_logo: found game '{}' with id {}", game.name, game_id);

        let images_url = format!("https://api.thegamesdb.net/v1/Games/Images?apikey={}&games_id={}", api_key, game_id);
        let images_response: ImagesResponse = client.get(&images_url).send().await.map_err(|e| format!("Errore nel recupero immagini TheGamesDB: {}", e))?.json().await.map_err(|e| format!("Errore nel parsing immagini TheGamesDB: {}", e))?;

    let game_images = images_response.data.images.get(&game_id.to_string()).ok_or("home.no_image_available_thegamesdb".to_string())?;

    let logo_image = game_images.iter().find(|img| img.image_type == "logo").or_else(|| game_images.iter().find(|img| img.image_type == "boxart")).ok_or("home.no_logo_or_boxart_thegamesdb".to_string())?;

        let image_url = format!("{}/{}", images_response.data.base_url.original.trim_end_matches('/'), logo_image.filename);
        println!("fetch_and_set_logo: downloading from TheGamesDB {}", image_url);

        let response = client.get(&image_url).send().await.map_err(|e| format!("Errore nel download TheGamesDB: {}", e))?;
        let bytes = response.bytes().await.map_err(|e| format!("Errore nella lettura bytes TheGamesDB: {}", e))?;
        println!("fetch_and_set_logo: downloaded {} bytes from TheGamesDB", bytes.len());

        let temp_path = format!("../data/temp_logo_{}.png", tableName);
        fs::write(&temp_path, &bytes).map_err(|e| format!("Errore scrittura temp TheGamesDB: {}", e))?;
        println!("fetch_and_set_logo: wrote temp file {}", temp_path);

        set_table_image(tableName, temp_path)
    }
}

#[tauri::command]
fn import_cht(filePath: String) -> Result<String, String> {
    // Estrarre nome tabella dal nome file (senza estensione)
    let path = Path::new(&filePath);
    let table_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("home.invalid_file_name".to_string())?;
    // Sanitize table name: replace non-alphanumeric (except _) with _
    let table_name: String = table_name.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Leggi il file .cht
    let content = fs::read_to_string(&filePath).map_err(|e| e.to_string())?;
    let mut lines = content.lines();
    // Skippa la prima riga (cheats = n)
    lines.next();

    // Parsing: raccogli tutti i cheat e i loro campi
    use std::collections::{BTreeSet, HashMap};
    let mut cheats: Vec<HashMap<String, String>> = Vec::new();
    let mut fields: BTreeSet<String> = BTreeSet::new();
    let mut current: HashMap<String, String> = HashMap::new();
    let mut last_idx = None;
    for line in lines {
        if let Some((k, v)) = line.split_once('=') {
            let k = k.trim();
            let v = v.trim().trim_matches('"');
            // Esempio: cheat0_desc
            if let Some(rest) = k.strip_prefix("cheat") {
                if let Some((idx, field)) = rest.split_once('_') {
                    if last_idx.is_some() && last_idx != Some(idx) {
                        // Nuovo cheat, pusha il precedente
                        cheats.push(current.clone());
                        current.clear();
                    }
                    last_idx = Some(idx);
                    current.insert(field.to_string(), v.to_string());
                    fields.insert(field.to_string());
                }
            }
        }
    }
    if !current.is_empty() {
        cheats.push(current);
    }

    // Aggiungi campo immagine
    // fields.insert("image".to_string()); // Non più necessario, immagini in table_images

    // Crea la tabella se non esiste
    let mut sql = format!(
        "CREATE TABLE IF NOT EXISTS `{}` (id INTEGER PRIMARY KEY AUTOINCREMENT",
        table_name
    );
    for field in &fields {
        sql.push_str(&format!(", `{}` TEXT", field));
    }
    sql.push_str(");");
    conn.execute(&sql, []).map_err(|e| e.to_string())?;

    // Inserisci i record
    for cheat in &cheats {
        let mut columns = Vec::new();
        let mut values: Vec<String> = Vec::new();
        for field in &fields {
            columns.push(field.as_str());
            values.push(cheat.get(field).cloned().unwrap_or_default());
        }
        let placeholders = vec!["?".to_string(); columns.len()].join(", ");
        let quoted_columns = columns.iter().map(|c| format!("`{}`", c)).collect::<Vec<_>>().join(", ");
        let insert_sql = format!(
            "INSERT INTO `{}` ({}) VALUES ({});",
            table_name,
            quoted_columns,
            placeholders
        );
        let params = params_from_iter(values.iter());
        conn.execute(&insert_sql, params)
            .map_err(|e| e.to_string())?;
    }

    Ok(format!(
        "Tabella '{}' creata/importata con {} record.",
        table_name,
        cheats.len()
    ))
}

#[tauri::command]
fn delete_table(table_name: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute(&format!("DROP TABLE `{}`;", table_name), []).map_err(|e| e.to_string())?;
    // Also delete from table_images
    conn.execute("DELETE FROM table_images WHERE table_name = ?", [table_name.clone()]).map_err(|e| e.to_string())?;
    Ok(format!("Tabella '{}' eliminata.", table_name))
}

#[tauri::command]
fn open_url(url: String) -> Result<String, String> {
    std::process::Command::new("cmd")
        .args(&["/C", "start", &url])
        .spawn()
        .map_err(|e| format!("Errore nell'apertura dell'URL: {}", e))?;
    Ok("URL aperto.".to_string())
}

#[tauri::command]
fn get_setting(key: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);", []).map_err(|e| e.to_string())?;
    let value: Option<String> = conn.query_row("SELECT value FROM settings WHERE key = ?", [key], |row| row.get(0)).unwrap_or(None);
    Ok(value.unwrap_or_default())
}

#[tauri::command]
fn set_setting(key: String, value: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);", []).map_err(|e| e.to_string())?;
    conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", params![key, value]).map_err(|e| e.to_string())?;
    Ok("Impostazione salvata.".to_string())
}

#[tauri::command]
fn update_record_order(tableName: String, recordOrder: Vec<String>) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Add order_index column if it doesn't exist and initialize existing records
    let alter_sql = format!("ALTER TABLE `{}` ADD COLUMN order_index INTEGER;", tableName);
    match conn.execute(&alter_sql, []) {
        Ok(_) => {
            // Column was added, initialize existing records with their current order (by id)
            conn.execute(&format!("UPDATE `{}` SET order_index = id;", tableName), []).map_err(|e| e.to_string())?;
        }
        Err(_) => {
            // Column already exists, continue
        }
    }

    // Update order_index for each record by id (recordOrder contains ids as strings)
    for (index, id_str) in recordOrder.iter().enumerate() {
    let id_num: i64 = id_str.parse::<i64>().map_err(|e: std::num::ParseIntError| e.to_string())?;
        conn.execute(
            &format!("UPDATE `{}` SET order_index = ? WHERE id = ?", tableName),
            params![index as i32, id_num]
        ).map_err(|e| e.to_string())?;
    }

    Ok("Ordine aggiornato.".to_string())
}

#[tauri::command]
fn export_cht_to_path(table_name: String, file_path: String) -> Result<String, String> {
    let db_path = "../data/cheats.db";
    fs::create_dir_all("../data").map_err(|e| e.to_string())?;
    let _conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Get all records from the table
    let records = get_records(table_name.clone())?;

    if records.is_empty() {
        return Err("home.table_empty".to_string());
    }

    // Get column names (excluding id and image)
    let columns = get_table_columns(table_name.clone())?;
    let export_columns: Vec<String> = columns.into_iter()
        .filter(|col| col != "id" && col != "image")
        .collect();

    // Create CHT content
    let mut content = format!("cheats = {}\n\n", records.len());

    for (i, record) in records.iter().enumerate() {
        for col in &export_columns {
            if let Some(value) = record.get(col) {
                // Escape quotes in value
                let escaped_value = value.replace("\"", "\\\"");
                content.push_str(&format!("cheat{}_{} = \"{}\"\n", i, col, escaped_value));
            }
        }
        // Add empty line between cheats (except for the last one)
        if i < records.len() - 1 {
            content.push_str("\n");
        }
    }

    // Write to file
    fs::write(&file_path, content).map_err(|e| format!("Errore nella scrittura del file: {}", e))?;

    // Try to open the directory containing the saved file
    if let Some(parent_dir) = Path::new(&file_path).parent() {
        if let Err(e) = std::process::Command::new("explorer").arg(parent_dir).spawn() {
            println!("Could not open explorer: {}", e);
        }
    }

    Ok(format!("File esportato con successo: {}", file_path))
}

#[derive(Deserialize)]
struct GamesResponse {
    data: GamesData,
}

#[derive(Deserialize)]
struct GamesData {
    games: Vec<Game>,
}

#[derive(Deserialize)]
struct Game {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct ImagesResponse {
    data: ImagesData,
}

#[derive(Deserialize)]
struct ImagesData {
    base_url: BaseUrl,
    images: HashMap<String, Vec<Image>>,
}

#[derive(Deserialize)]
struct BaseUrl {
    original: String,
}

#[derive(Deserialize)]
struct RawgResponse {
    results: Vec<RawgGame>,
}

#[derive(Deserialize)]
struct RawgGame {
    id: u32,
    name: String,
    background_image: Option<String>,
}

#[derive(Deserialize)]
struct Image {
    id: u32,
    #[serde(rename = "type")]
    image_type: String,
    filename: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![import_cht, get_tables, delete_table, get_records, set_table_image, delete_table_image, update_record, delete_record, insert_record, get_table_columns, fetch_and_set_logo, get_setting, set_setting, open_url, export_cht_to_path, update_record_order])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
