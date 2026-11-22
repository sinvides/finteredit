use mod_api::{
    Command, CommandArgumentValue, CommandSource, Flow, ItemStack, RegisterCommandsEvent,
    RunModCommandEvent, ServerHost,
};

use crate::selection::{points, volume};
use crate::{blocks, editor, i18n, AXE};

pub fn register(event: &mut RegisterCommandsEvent<'_>) {
    event.commands.push(Command::new("wand"));
    event.commands.push(Command::new("/wand"));
    event.commands.push(Command::new("/set <n:text>"));
    event.commands.push(Command::new("undo"));
    event.commands.push(Command::new("/undo"));
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
                let _ = player.give_stack(host, ItemStack::of(*axe));
            }
            points().lock().unwrap().remove(&uuid);
            Flow::Cancel
        }
        Some("/set") => {
            let name = match event.command.argument("n") {
                Some(CommandArgumentValue::String(value)) => value.as_str(),
                _ => return Flow::Cancel,
            };
            if blocks::resolve(host, name).is_none() {
                i18n::tell(host, player, i18n::unknown_block(name));
                return Flow::Cancel;
            }
            let selection = match points().lock().unwrap().get(&uuid).copied() {
                Some(selection) => selection,
                None => {
                    i18n::tell(host, player, i18n::no_selection());
                    return Flow::Cancel;
                }
            };
            let second = match selection.second {
                Some(second) => second,
                None => {
                    i18n::tell(host, player, i18n::no_second_point());
                    return Flow::Cancel;
                }
            };
            let state = match blocks::resolve(host, name) {
                Some(state) => state,
                None => return Flow::Cancel,
            };
            let level = match player.level(host) {
                Some(level) => level,
                None => return Flow::Cancel,
            };
            editor::fill(host, level, uuid, selection, state);
            i18n::tell(host, player, i18n::filled(volume(selection.first, second)));
            Flow::Cancel
        }
        Some("undo") | Some("/undo") => {
            let level = match player.level(host) {
                Some(level) => level,
                None => return Flow::Cancel,
            };
            match editor::restore(host, uuid, level) {
                Some(count) => i18n::tell(host, player, i18n::undone(count)),
                None => i18n::tell(host, player, i18n::no_undo()),
            }
            Flow::Cancel
        }
        _ => Flow::Continue,
    }
}
