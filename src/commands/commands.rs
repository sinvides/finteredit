use mod_api::{
    Command, CommandArgumentValue, CommandSource, Flow, RegisterCommandsEvent, RunModCommandEvent,
    ServerHost,
};

use crate::core::selection::{dimension, points, volume, MAX_VOLUME};
use crate::{core::blocks, core::editor, i18n, interact::wand, AXE};

fn block_pos(position: [f64; 3]) -> [i32; 3] {
    [
        position[0].floor() as i32,
        position[1].floor() as i32,
        position[2].floor() as i32,
    ]
}

pub fn register(event: &mut RegisterCommandsEvent<'_>) {
    event.commands.push(Command::new("wand"));
    event.commands.push(Command::new("/wand"));
    event.commands.push(Command::op("set <n:text>", 2));
    event.commands.push(Command::op("/set <n:text>", 2));
    event.commands.push(Command::op("/copy", 2));
    event.commands.push(Command::op("/paste [o:word]", 2));
    event.commands.push(Command::op("undo", 2));
    event.commands.push(Command::op("/undo", 2));
}

pub fn run(host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
    let player = match event.source {
        CommandSource::Player { player, .. } => *player,
        _ => return Flow::Continue,
    };
    let uuid = player.uuid();
    match event.command.literals.first().map(String::as_str) {
        Some("wand") | Some("/wand") => {
            if let Some(axe) = AXE.get() {
                let _ = player.give_stack(host, wand::create(*axe));
            }
            points().lock().unwrap().remove(&uuid);
            Flow::Cancel
        }
        Some("set") | Some("/set") => {
            let name = match event.command.argument("n") {
                Some(CommandArgumentValue::String(value)) => value.as_str(),
                _ => return Flow::Cancel,
            };
            let state = match blocks::resolve(host, name) {
                Some(state) => state,
                None => {
                    i18n::tell(host, player, i18n::unknown_block(name));
                    return Flow::Cancel;
                }
            };
            let current = match dimension(host, player) {
                Some(current) => current,
                None => return Flow::Cancel,
            };
            let selection = match points().lock().unwrap().get(&uuid).cloned() {
                Some(selection) => selection.clone(),
                None => {
                    i18n::tell(host, player, i18n::no_selection());
                    return Flow::Cancel;
                }
            };
            if selection.dimension != current {
                i18n::tell(host, player, i18n::wrong_dimension());
                return Flow::Cancel;
            }
            let second = match selection.second {
                Some(second) => second,
                None => {
                    i18n::tell(host, player, i18n::no_second_point());
                    return Flow::Cancel;
                }
            };
            let count = volume(selection.first, second);
            if count > MAX_VOLUME {
                i18n::tell(host, player, i18n::too_large(MAX_VOLUME));
                return Flow::Cancel;
            }
            let level = match player.level(host) {
                Some(level) => level,
                None => return Flow::Cancel,
            };
            editor::fill(host, level, uuid, selection, state);
            i18n::tell(host, player, i18n::filled(count));
            Flow::Cancel
        }
        Some("/copy") => {
            let current = match dimension(host, player) {
                Some(current) => current,
                None => return Flow::Cancel,
            };
            let selection = match points().lock().unwrap().get(&uuid).cloned() {
                Some(selection) => selection,
                None => {
                    i18n::tell(host, player, i18n::no_selection());
                    return Flow::Cancel;
                }
            };
            if selection.dimension != current {
                i18n::tell(host, player, i18n::wrong_dimension());
                return Flow::Cancel;
            }
            let second = match selection.second {
                Some(second) => second,
                None => {
                    i18n::tell(host, player, i18n::no_second_point());
                    return Flow::Cancel;
                }
            };
            let count = volume(selection.first, second);
            if count > MAX_VOLUME {
                i18n::tell(host, player, i18n::too_large(MAX_VOLUME));
                return Flow::Cancel;
            }
            let level = match player.level(host) {
                Some(level) => level,
                None => return Flow::Cancel,
            };
            let copied = editor::copy(
                host,
                level,
                uuid,
                selection,
                block_pos(event.command.position),
                &current,
            );
            i18n::tell(host, player, i18n::copied(copied));
            Flow::Cancel
        }
        Some("/paste") => {
            let current = match dimension(host, player) {
                Some(current) => current,
                None => return Flow::Cancel,
            };
            let original = match event.command.argument("o") {
                Some(CommandArgumentValue::String(value)) => value == "-o",
                _ => false,
            };
            let level = match player.level(host) {
                Some(level) => level,
                None => return Flow::Cancel,
            };
            match editor::paste(
                host,
                level,
                uuid,
                block_pos(event.command.position),
                original,
                &current,
            ) {
                Some(count) => i18n::tell(host, player, i18n::pasted(count)),
                None => i18n::tell(host, player, i18n::no_clipboard()),
            }
            Flow::Cancel
        }
        Some("undo") | Some("/undo") => {
            let current = match dimension(host, player) {
                Some(current) => current,
                None => return Flow::Cancel,
            };
            let level = match player.level(host) {
                Some(level) => level,
                None => return Flow::Cancel,
            };
            match editor::restore(host, uuid, level, &current) {
                Ok((restored, failed)) => i18n::tell(host, player, i18n::undone(restored, failed)),
                Err(editor::UndoError::Empty) => i18n::tell(host, player, i18n::no_undo()),
                Err(editor::UndoError::WrongDimension) => i18n::tell(host, player, i18n::wrong_dimension()),
            }
            Flow::Cancel
        }
        _ => Flow::Continue,
    }
}