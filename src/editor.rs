use mod_api::{BlockState, BlockUpdateFlags, LevelRef, ServerHost};

use crate::selection::{clipboards, each_pos, push_undo, undo, Clipboard, Selection, UndoEntry};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UndoError {
    Empty,
    WrongDimension,
}

fn flags() -> BlockUpdateFlags {
    BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS
}

pub fn fill(host: &mut dyn ServerHost, level: LevelRef, uuid: u128, selection: Selection, state: BlockState) {
    let second = match selection.second {
        Some(second) => second,
        None => return,
    };
    let mut blocks = Vec::new();
    each_pos(selection.first, second, |pos| {
        let old = level.block_state(host, pos);
        if matches!(level.set_block(host, pos, state, flags()), Ok(true)) {
            if let Some(previous) = old {
                blocks.push((pos, previous));
            }
        }
    });
    push_undo(
        uuid,
        UndoEntry {
            dimension: selection.dimension.clone(),
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
        if let Some(state) = level.block_state(host, pos) {
            blocks.push((
                [pos[0] - origin[0], pos[1] - origin[1], pos[2] - origin[2]],
                state,
            ));
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
    for (relative, state) in clipboard.blocks {
        let pos = [
            base[0] + relative[0],
            base[1] + relative[1],
            base[2] + relative[2],
        ];
        let old = level.block_state(host, pos);
        if matches!(level.set_block(host, pos, state, flags()), Ok(true)) {
            if let Some(previous) = old {
                undo_blocks.push((pos, previous));
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

pub fn restore(host: &mut dyn ServerHost, uuid: u128, level: LevelRef, dimension: &str) -> Result<usize, UndoError> {
    let entry = {
        let mut history = undo().lock().unwrap();
        let entries = history.get_mut(&uuid).ok_or(UndoError::Empty)?;
        let last = entries.last().ok_or(UndoError::Empty)?;
        if last.dimension != dimension {
            return Err(UndoError::WrongDimension);
        }
        entries.pop().ok_or(UndoError::Empty)?
    };
    let count = entry.blocks.len();
    for (pos, state) in entry.blocks {
        let _ = level.set_block(host, pos, state, flags());
    }
    Ok(count)
}
