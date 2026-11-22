use mod_api::{BlockState, BlockUpdateFlags, LevelRef, ServerHost};

use crate::selection::{each_pos, undo, Selection};

fn flags() -> BlockUpdateFlags {
    BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS
}

pub fn fill(host: &mut dyn ServerHost, level: LevelRef, uuid: u128, selection: Selection, state: BlockState) {
    let second = match selection.second {
        Some(second) => second,
        None => return,
    };
    let mut snapshot = Vec::new();
    each_pos(selection.first, second, |pos| {
        if let Some(old) = level.block_state(host, pos) {
            snapshot.push((pos, old));
        }
    });
    each_pos(selection.first, second, |pos| {
        let _ = level.set_block(host, pos, state, flags());
    });
    undo().lock().unwrap().entry(uuid).or_default().push(snapshot);
}

pub fn restore(host: &mut dyn ServerHost, uuid: u128, level: LevelRef) -> Option<usize> {
    let snapshot = undo().lock().unwrap().get_mut(&uuid).and_then(Vec::pop)?;
    let count = snapshot.len();
    for (pos, state) in snapshot {
        let _ = level.set_block(host, pos, state, flags());
    }
    Some(count)
}
