use mod_api::{BlockState, ServerHost};

pub fn resolve(host: &dyn ServerHost, name: &str) -> Option<BlockState> {
    let id = match name {
        "0" => "minecraft:air",
        "1" => "minecraft:stone",
        "2" => "minecraft:grass_block",
        "3" => "minecraft:dirt",
        "4" => "minecraft:cobblestone",
        "5:1" => "minecraft:oak_planks",
        "5:2" => "minecraft:spruce_planks",
        "5:3" => "minecraft:birch_planks",
        "5:4" => "minecraft:jungle_planks",
        "5:5" => "minecraft:acacia_planks",
        "6" => "minecraft:bedrock",
        _ => return None,
    };
    host.block_state_id(id)
}
