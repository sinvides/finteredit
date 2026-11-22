use mod_api::{Flow, HostSide, ItemKey, LeftClickBlockEvent, PlayerRef, RightClickBlockEvent, ServerHost};

use crate::selection::{points, volume, Selection};
use crate::{i18n, AXE};

fn has_axe(host: &dyn ServerHost, player: PlayerRef, axe: ItemKey) -> bool {
    player
        .selected_slot(host)
        .and_then(|slot| {
            player
                .inventory_slots(host)
                .and_then(|slots| slots.into_iter().find(|(s, _)| *s == slot as u32).map(|(_, stack)| stack))
        })
        .is_some_and(|stack| stack.item() == axe)
}

pub fn left(host: &mut dyn ServerHost, event: &mut LeftClickBlockEvent) -> Flow {
    let player = match host.players().acting_player() {
        Some(player) => player,
        None => return Flow::Continue,
    };
    let axe = match AXE.get() {
        Some(axe) => *axe,
        None => return Flow::Continue,
    };
    if !has_axe(host, player, axe) {
        return Flow::Continue;
    }
    let pos = event.pos;
    points().lock().unwrap().insert(player.uuid(), Selection { first: pos, second: None });
    i18n::tell(host, player, i18n::first_point(pos));
    Flow::Cancel
}

pub fn right(side: HostSide<'_>, event: &mut RightClickBlockEvent<'_>) -> Flow {
    let host = match side {
        HostSide::Server(host) => host,
        HostSide::Client(_) => return Flow::Continue,
    };
    let player = match host.players().acting_player() {
        Some(player) => player,
        None => return Flow::Continue,
    };
    let axe = match AXE.get() {
        Some(axe) => *axe,
        None => return Flow::Continue,
    };
    if !has_axe(host, player, axe) {
        return Flow::Continue;
    }
    let pos = event.click.pos;
    let count = {
        let mut points = points().lock().unwrap();
        match points.get_mut(&player.uuid()) {
            Some(selection) if selection.second.is_none() => {
                let first = selection.first;
                selection.second = Some(pos);
                volume(first, pos)
            }
            _ => return Flow::Continue,
        }
    };
    i18n::tell(host, player, i18n::second_point(pos, count));
    Flow::Cancel
}
