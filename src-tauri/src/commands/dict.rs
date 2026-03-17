use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

/// Dictionary entry structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DictEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub translation: Option<String>,
    pub pos: Option<String>,
    pub collins: Option<i32>,
    pub oxford: Option<i32>,
    pub tag: Option<String>,
    pub bnc: Option<i32>,
    pub frq: Option<i32>,
    pub exchange: Option<String>,
    pub detail: Option<String>,
    pub audio: Option<String>,
}

/// State for managing database connection
pub struct DictState {
    pub conn: Mutex<Connection>,
}

impl DictState {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

/// Query dictionary by exact word match
#[tauri::command]
pub fn query_dict(word: &str, state: State<'_, DictState>) -> Result<Option<DictEntry>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT word, phonetic, definition, translation, pos, collins, oxford, tag, bnc, frq, exchange, detail, audio FROM dict WHERE word = ?1")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_row(params![word.to_lowercase()], |row| {
            Ok(DictEntry {
                word: row.get(0)?,
                phonetic: row.get(1)?,
                definition: row.get(2)?,
                translation: row.get(3)?,
                pos: row.get(4)?,
                collins: row.get(5)?,
                oxford: row.get(6)?,
                tag: row.get(7)?,
                bnc: row.get(8)?,
                frq: row.get(9)?,
                exchange: row.get(10)?,
                detail: row.get(11)?,
                audio: row.get(12)?,
            })
        });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Query dictionary by translation (Chinese to English)
#[tauri::command]
pub fn query_dict_by_translation(translation: &str, state: State<'_, DictState>) -> Result<Vec<DictEntry>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    // Search for translations containing the keyword, order by collins and frq
    let mut stmt = conn
        .prepare("SELECT word, phonetic, definition, translation, pos, collins, oxford, tag, bnc, frq, exchange, detail, audio FROM dict WHERE translation LIKE ?1 ORDER BY collins DESC, frq DESC LIMIT 20")
        .map_err(|e| e.to_string())?;

    let pattern = format!("%{}%", translation);
    let entries = stmt
        .query_map(params![pattern], |row| {
            Ok(DictEntry {
                word: row.get(0)?,
                phonetic: row.get(1)?,
                definition: row.get(2)?,
                translation: row.get(3)?,
                pos: row.get(4)?,
                collins: row.get(5)?,
                oxford: row.get(6)?,
                tag: row.get(7)?,
                bnc: row.get(8)?,
                frq: row.get(9)?,
                exchange: row.get(10)?,
                detail: row.get(11)?,
                audio: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let results: Vec<DictEntry> = entries.filter_map(|e| e.ok()).collect();
    Ok(results)
}

/// Insert or update dictionary entry (for caching API results)
#[tauri::command]
pub fn save_dict_entry(entry: DictEntry, state: State<'_, DictState>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO dict (word, phonetic, definition, translation, pos, collins, oxford, tag, bnc, frq, exchange, detail, audio) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            entry.word,
            entry.phonetic,
            entry.definition,
            entry.translation,
            entry.pos,
            entry.collins,
            entry.oxford,
            entry.tag,
            entry.bnc,
            entry.frq,
            entry.exchange,
            entry.detail,
            entry.audio,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
