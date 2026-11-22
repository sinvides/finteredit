use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use mod_api::BlockState;

#[derive(Clone, Copy)]
pub struct Selection {
    pub first: [i32; 3],
    pub second: Option<[i32; 3]>,
}

type Points = HashMap<u128, Selection>;
type History = HashMap<u128, Vec<Vec<([i32; 3], BlockState)>>>;

static POINTS: OnceLock<Mutex<Points>> = OnceLock::new();
static UNDO: OnceLock<Mutex<History>> = OnceLock::new();

pub fn points() -> &'static Mutex<Points> {
    POINTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn undo() -> &'static Mutex<History> {
    UNDO.get_or_init(|| Mutex::new(HashMap::new()))
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
