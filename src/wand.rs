use mod_api::{Component, Compound, ItemKey, ItemStack, Tag};

struct Wand;

impl Component for Wand {
    type Value = Tag;

    fn id(&self) -> &str {
        "minecraft:custom_data"
    }

    fn decode(&self, value: &Tag) -> Option<Self::Value> {
        Some(value.clone())
    }

    fn encode(&self, value: Self::Value) -> Tag {
        value
    }
}

pub fn create(key: ItemKey) -> ItemStack {
    let mut stack = ItemStack::of(key);
    let mut data = Compound::new();
    data.insert("finteredit_wand".into(), Tag::Byte(1));
    let _ = stack.set(&Wand, Tag::Compound(data));
    stack
}

pub fn is_wand(stack: &ItemStack) -> bool {
    match stack.get(&Wand) {
        Some(Tag::Compound(data)) => data.keys().any(|key| key.to_string_lossy() == "finteredit_wand"),
        _ => false,
    }
}
