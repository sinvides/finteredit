mod blocks;
mod commands;
mod editor;
mod i18n;
mod interact;
mod selection;
mod suggest;

use std::sync::OnceLock;

use mod_api::{
    Flow, Hooks, HostSide, ItemKey, LeftClickBlockEvent, PlayerLoggedOutEvent, Registrar,
    RegisterCommandsEvent, RegistrationContext, RightClickBlockEvent, RunModCommandEvent, ServerHost,
    TabCompleteEvent,
};

pub(crate) static AXE: OnceLock<ItemKey> = OnceLock::new();

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        let _ = AXE.set(r.item_key("minecraft:wooden_axe"));
    }

    fn register_commands(&self, _: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        commands::register(event);
    }

    fn run_mod_command(&self, host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
        commands::run(host, event)
    }

    fn left_click_block(&self, host: &mut dyn ServerHost, event: &mut LeftClickBlockEvent) -> Flow {
        interact::left(host, event)
    }

    fn right_click_block(&self, side: HostSide<'_>, event: &mut RightClickBlockEvent<'_>) -> Flow {
        interact::right(side, event)
    }

    fn player_logged_out(&self, _: &mut dyn ServerHost, event: &mut PlayerLoggedOutEvent<'_>) {
        selection::clear_player(event.player);
    }

    fn tab_complete(&self, host: &mut dyn ServerHost, event: &mut TabCompleteEvent<'_>) {
        suggest::blocks(host, event);
    }
}
