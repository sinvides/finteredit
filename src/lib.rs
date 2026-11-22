use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use mod_api::text::TextStyle;
use mod_api::{
    self, BlockState, BlockUpdateFlags, Command, CommandArgumentValue, CommandSource, Flow, Hooks,
    HostSide, ItemKey, ItemStack, LeftClickBlockEvent, PlayerRef, Registrar, RegisterCommandsEvent,
    RegistrationContext, RightClickBlockEvent, RunModCommandEvent, ServerHost, Text,
};

static AXE: OnceLock<ItemKey> = OnceLock::new();
static POINTS: OnceLock<Mutex<HashMap<u128, Selection>>> = OnceLock::new();
static UNDO: OnceLock<Mutex<HashMap<u128, Vec<Vec<([i32; 3], BlockState)>>>>> = OnceLock::new();

#[derive(Clone, Copy)]
struct Selection {
    first: [i32; 3],
    second: Option<[i32; 3]>,
}

fn points() -> &'static Mutex<HashMap<u128, Selection>> {
    POINTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn undo() -> &'static Mutex<HashMap<u128, Vec<Vec<([i32; 3], BlockState)>>>> {
    UNDO.get_or_init(|| Mutex::new(HashMap::new()))
}

fn volume(a: [i32; 3], b: [i32; 3]) -> i64 {
    let dx = (a[0] - b[0]).abs() as i64 + 1;
    let dy = (a[1] - b[1]).abs() as i64 + 1;
    let dz = (a[2] - b[2]).abs() as i64 + 1;
    dx * dy * dz
}

fn block_name(n: &str) -> Option<&'static str> {
    match n {
        "0" => Some("minecraft:air"),
        "1" => Some("minecraft:stone"),
        "2" => Some("minecraft:grass_block"),
        "3" => Some("minecraft:dirt"),
        "4" => Some("minecraft:cobblestone"),
        "5:1" => Some("minecraft:oak_planks"),
        "5:2" => Some("minecraft:spruce_planks"),
        "5:3" => Some("minecraft:birch_planks"),
        "5:4" => Some("minecraft:jungle_planks"),
        "5:5" => Some("minecraft:acacia_planks"),
        "6" => Some("minecraft:bedrock"),
        _ => None,
    }
}

fn tell(host: &mut dyn ServerHost, player: PlayerRef, line: String) {
    let mut style = TextStyle::default();
    style.color = Some("#CC6600".to_owned());
    let _ = player.tell(host, Text::literal(line).with_style(style));
}

fn flags() -> BlockUpdateFlags {
    BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS
}

fn each_pos(a: [i32; 3], b: [i32; 3], mut visit: impl FnMut([i32; 3])) {
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

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        let _ = AXE.set(r.item_key("minecraft:wooden_axe"));
    }

    fn register_commands(&self, _: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        event.commands.push(Command::new("wand"));
        event.commands.push(Command::new("/wand"));
        event.commands.push(Command::new("/set <n:text>"));
        event.commands.push(Command::new("undo"));
        event.commands.push(Command::new("/undo"));
    }

    fn run_mod_command(&self, host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
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
                let n = match event.command.argument("n") {
                    Some(CommandArgumentValue::String(value)) => value.as_str(),
                    _ => return Flow::Cancel,
                };
                let name = match block_name(n) {
                    Some(name) => name,
                    None => {
                        tell(host, player, format!("Неизвестный блок: {}", n));
                        return Flow::Cancel;
                    }
                };
                let selection = match points().lock().unwrap().get(&uuid).copied() {
                    Some(selection) => selection,
                    None => {
                        tell(host, player, "Сначала выделите две точки.".to_owned());
                        return Flow::Cancel;
                    }
                };
                let second = match selection.second {
                    Some(second) => second,
                    None => {
                        tell(host, player, "Выделите вторую точку.".to_owned());
                        return Flow::Cancel;
                    }
                };
                let state = match host.block_state_id(name) {
                    Some(state) => state,
                    None => return Flow::Cancel,
                };
                let level = match player.level(host) {
                    Some(level) => level,
                    None => return Flow::Cancel,
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
                tell(host, player, format!("Застроено блоков: {}", volume(selection.first, second)));
                Flow::Cancel
            }
            Some("undo") | Some("/undo") => {
                let snapshot = undo().lock().unwrap().get_mut(&uuid).and_then(Vec::pop);
                let snapshot = match snapshot {
                    Some(snapshot) => snapshot,
                    None => {
                        tell(host, player, "Нечего отменять.".to_owned());
                        return Flow::Cancel;
                    }
                };
                let level = match player.level(host) {
                    Some(level) => level,
                    None => return Flow::Cancel,
                };
                let count = snapshot.len();
                for (pos, state) in snapshot {
                    let _ = level.set_block(host, pos, state, flags());
                }
                tell(host, player, format!("Отменено блоков: {}", count));
                Flow::Cancel
            }
            _ => Flow::Continue,
        }
    }

    fn left_click_block(&self, host: &mut dyn ServerHost, event: &mut LeftClickBlockEvent) -> Flow {
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
        tell(
            host,
            player,
            format!("Первая точка установлена: ({}, {}, {}) (1 блоков)", pos[0], pos[1], pos[2]),
        );
        Flow::Cancel
    }

    fn right_click_block(&self, side: HostSide<'_>, event: &mut RightClickBlockEvent<'_>) -> Flow {
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
        let uuid = player.uuid();
        let pos = event.click.pos;
        let mut points = points().lock().unwrap();
        let count = match points.get_mut(&uuid) {
            Some(selection) if selection.second.is_none() => {
                let first = selection.first;
                selection.second = Some(pos);
                volume(first, pos)
            }
            _ => return Flow::Continue,
        };
        drop(points);
        tell(
            host,
            player,
            format!("Вторая точка установлена: ({}, {}, {}) ({} блоков)", pos[0], pos[1], pos[2], count),
        );
        Flow::Cancel
    }
}
