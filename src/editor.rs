use mod_api::{BlockState, BlockUpdateFlags, LevelRef, ServerHost};

use crate::selection::{each_pos, push_undo, undo, Selection, UndoEntry};

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
