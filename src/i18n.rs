use mod_api::{PlayerRef, ServerHost, Text};

fn arg(value: impl ToString) -> Text {
    Text::literal(value.to_string())
}

pub fn tell(host: &mut dyn ServerHost, player: PlayerRef, mut text: Text) {
    text.style.color = Some("#CC6600".to_owned());
    let _ = player.tell(host, text);
}

pub fn first_point(pos: [i32; 3]) -> Text {
    Text::translate(
        "finteredit.point.first",
        vec![arg(pos[0]), arg(pos[1]), arg(pos[2]), arg(1)],
    )
}

pub fn second_point(pos: [i32; 3], count: i64) -> Text {
    Text::translate(
        "finteredit.point.second",
        vec![arg(pos[0]), arg(pos[1]), arg(pos[2]), arg(count)],
    )
}

pub fn filled(count: i64) -> Text {
    Text::translate("finteredit.fill.done", vec![arg(count)])
}

pub fn undone(count: usize) -> Text {
    Text::translate("finteredit.undo.done", vec![arg(count)])
}

pub fn unknown_block(name: &str) -> Text {
    Text::translate("finteredit.error.unknown_block", vec![arg(name)])
}

pub fn no_selection() -> Text {
    Text::translate("finteredit.error.no_selection", vec![])
}

pub fn no_second_point() -> Text {
    Text::translate("finteredit.error.no_second", vec![])
}

pub fn no_undo() -> Text {
    Text::translate("finteredit.error.no_undo", vec![])
}

pub fn too_large(max: i64) -> Text {
    Text::translate("finteredit.error.too_large", vec![arg(max)])
}

pub fn wrong_dimension() -> Text {
    Text::translate("finteredit.error.dimension", vec![])
}
