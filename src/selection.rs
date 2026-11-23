use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use mod_api::{BlockState, Compound, PlayerRef, ServerHost};

pub const MAX_VOLUME: i64 = 32768;
pub const MAX_UNDO_DEPTH: usize = 8;

#[derive(Clone)]
pub struct Selection {
    pub first: [i32; 3],
    pub second: Option<[i32; 3]>,
    pub dimension: String,
}

#[derive(Clone)]
pub struct SavedBlock {
    pub pos: [i32; 3],
    pub state: BlockState,
    pub data: Option<Compound>,
}

#[derive(Clone)]
pub struct ClipboardBlock {
    pub relative: [i32; 3],
    pub state: BlockState,
    pub data: Option<Compound>,
}

pub struct UndoEntry {
    pub dimension: String,
    pub blocks: Vec<SavedBlock>,
}

#[derive(Clone)]
pub struct Clipboard {
    pub dimension: String,
    pub origin: [i32; 3],
    pub offset: [i32; 3],
    pub blocks: Vec<ClipboardBlock>,
}

type Points = HashMap<u128, Selection>;
type History = HashMap<u128, Vec<UndoEntry>>;
type Clipboards = HashMap<u128, Clipboard>;

static POINTS: OnceLock<Mutex<Points>> = OnceLock::new();
static UNDO: OnceLock<Mutex<History>> = OnceLock::new();
static CLIPBOARDS: OnceLock<Mutex<Clipboards>> = OnceLock::new();

pub fn points() -> &'static Mutex<Points> {
    POINTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn undo() -> &'static Mutex<History> {
    UNDO.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn clipboards() -> &'static Mutex<Clipboards> {
    CLIPBOARDS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn dimension(host: &dyn ServerHost, player: PlayerRef) -> Option<String> {
    player.dimension(host).map(|value| value.to_string())
}

pub fn clear_player(uuid: u128) {
    points().lock().unwrap().remove(&uuid);
    undo().lock().unwrap().remove(&uuid);
    clipboards().lock().unwrap().remove(&uuid);
}

pub fn push_undo(uuid: u128, entry: UndoEntry) {
    let mut history = undo().lock().unwrap();
    let entries = history.entry(uuid).or_default();
    if entries.len() >= MAX_UNDO_DEPTH {
        entries.remove(0);
    }
    entries.push(entry);
}

pub fn volume(a: [i32; 3], b: [i32; 3]) -> i64 {
    let dx = (a[0] - b[0]).abs() as i64 + 1;
    let dy = (a[1] - b[1]).abs() as i64 + 1;
    let dz = (a[2] - b[2]).abs() as i64 + 1;
    dx * dy * dz
}

pub fn each_pos(a: [i32; 3], b: [i32; 3], mut visit: impl FnMut([i32; 3])) {
    let min = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
    let max = [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])];
    for x in min[0]..=max[0] {
        for y in min[1]..=max[1] {
            for z in min[2]..=max[2] {
                visit([x, y, z]);
            }
        }
    }
}
