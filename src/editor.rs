use mod_api::{BlockUpdateFlags, Host, HostBlockStates, LevelRef, ServerHost};

use crate::selection::{
    clipboards, each_pos, push_undo, undo, Clipboard, ClipboardBlock, SavedBlock, Selection, UndoEntry,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UndoError {
    Empty,
    WrongDimension,
}

fn flags() -> BlockUpdateFlags {
    BlockUpdateFlags::NEIGHBORS
        | BlockUpdateFlags::CLIENTS
        | BlockUpdateFlags::SKIP_BLOCK_ENTITY_SIDE_EFFECTS
}

fn saved_block(host: &dyn Host, level: LevelRef, pos: [i32; 3]) -> Option<SavedBlock> {
    let state = level.block_state(host, pos)?;
    let data = match host.block_entity(level, pos) {
        Some(entity) => Some(entity.data(host)?),
        None => None,
    };
    Some(SavedBlock { pos, state, data })
}

fn can_write(host: &dyn Host, level: LevelRef, pos: [i32; 3]) -> bool {
    if host.block_entity(level, pos).is_some() {
        return false;
    }
    match level
        .block_state(host, pos)
        .and_then(|state| host.block_states().has_block_entity(state))
    {
        Some(true) => false,
        _ => true,
    }
}

fn apply(host: &mut dyn ServerHost, level: LevelRef, saved: &SavedBlock) -> bool {
    if !can_write(host, level, saved.pos) {
        return false;
    }
    if !matches!(level.set_block(host, saved.pos, saved.state, flags()), Ok(true)) {
        return false;
    }
    if let Some(data) = &saved.data {
        if let Some(entity) = host.block_entity(level, saved.pos) {
            let _ = entity.replace_data(host, data.clone());
            let _ = entity.set_changed(host);
        }
    }
    true
}

pub fn fill(host: &mut dyn ServerHost, level: LevelRef, uuid: u128, selection: Selection, state: mod_api::BlockState) {
    let second = match selection.second {
        Some(second) => second,
        None => return,
    };
    let mut blocks = Vec::new();
    each_pos(selection.first, second, |pos| {
        if !can_write(host, level, pos) {
            return;
        }
        let old = saved_block(host, level, pos);
        if matches!(level.set_block(host, pos, state, flags()), Ok(true)) {
            if let Some(old) = old {
                blocks.push(old);
            }
        }
    });
    push_undo(
        uuid,
        UndoEntry {
            dimension: selection.dimension,
            blocks,
        },
    );
}

pub fn copy(
    host: &mut dyn ServerHost,
    level: LevelRef,
    uuid: u128,
    selection: Selection,
    player_pos: [i32; 3],
    dimension: &str,
) -> usize {
    let second = match selection.second {
        Some(second) => second,
        None => return 0,
    };
    let origin = [
        selection.first[0].min(second[0]),
        selection.first[1].min(second[1]),
        selection.first[2].min(second[2]),
    ];
    let mut blocks = Vec::new();
    each_pos(selection.first, second, |pos| {
        if let Some(saved) = saved_block(host, level, pos) {
            blocks.push(ClipboardBlock {
                relative: [pos[0] - origin[0], pos[1] - origin[1], pos[2] - origin[2]],
                state: saved.state,
                data: saved.data,
            });
        }
    });
    let count = blocks.len();
    let offset = [
        origin[0] - player_pos[0],
        origin[1] - player_pos[1],
        origin[2] - player_pos[2],
    ];
    clipboards().lock().unwrap().insert(
        uuid,
        Clipboard {
            dimension: dimension.to_owned(),
            origin,
            offset,
            blocks,
        },
    );
    count
}

pub fn paste(
    host: &mut dyn ServerHost,
    level: LevelRef,
    uuid: u128,
    current_pos: [i32; 3],
    original: bool,
    dimension: &str,
) -> Option<usize> {
    let clipboard = clipboards().lock().unwrap().get(&uuid).cloned()?;
    if clipboard.dimension != dimension {
        return None;
    }
    let base = if original {
        clipboard.origin
    } else {
        [
            current_pos[0] + clipboard.offset[0],
            current_pos[1] + clipboard.offset[1],
            current_pos[2] + clipboard.offset[2],
        ]
    };
    let mut undo_blocks = Vec::new();
    let mut count = 0;
    for block in clipboard.blocks {
        let pos = [
            base[0] + block.relative[0],
            base[1] + block.relative[1],
            base[2] + block.relative[2],
        ];
        if !can_write(host, level, pos) {
            continue;
        }
        let old = saved_block(host, level, pos);
        if matches!(level.set_block(host, pos, block.state, flags()), Ok(true)) {
            if let Some(data) = block.data {
                if let Some(entity) = host.block_entity(level, pos) {
                    let _ = entity.replace_data(host, data);
                    let _ = entity.set_changed(host);
                }
            }
            if let Some(old) = old {
                undo_blocks.push(old);
            }
            count += 1;
        }
    }
    push_undo(
        uuid,
        UndoEntry {
            dimension: dimension.to_owned(),
            blocks: undo_blocks,
        },
    );
    Some(count)
}

pub fn restore(
    host: &mut dyn ServerHost,
    uuid: u128,
    level: LevelRef,
    dimension: &str,
) -> Result<(usize, usize), UndoError> {
    let mut entry = {
        let mut history = undo().lock().unwrap();
        let entries = history.get_mut(&uuid).ok_or(UndoError::Empty)?;
        let last = entries.last().ok_or(UndoError::Empty)?;
        if last.dimension != dimension {
            return Err(UndoError::WrongDimension);
        }
        entries.pop().ok_or(UndoError::Empty)?
    };
    let mut failed = Vec::new();
    let mut restored = 0;
    for saved in entry.blocks {
        if apply(host, level, &saved) {
            restored += 1;
        } else {
            failed.push(saved);
        }
    }
    let remaining = failed.len();
    if !failed.is_empty() {
        entry.blocks = failed;
        push_undo(uuid, entry);
    }
    Ok((restored, remaining))
}
