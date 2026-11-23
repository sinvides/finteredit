use mod_api::{Suggestion, TabCompleteEvent, ServerHost};

pub fn blocks(host: &mut dyn ServerHost, event: &mut TabCompleteEvent<'_>) {
    let command = event.command.trim_start_matches('/');
    if !command.starts_with("set ") {
        return;
    }
    let range = (event.range[0], event.range[1]);
    let prefix = event.command.get(range.0..range.1).unwrap_or("");
    event.suggestions.clear();
    for key in host.block_keys() {
        let id = key.id(host);
        if id.starts_with(prefix) {
            event.suggestions.push(Suggestion::new(range, id));
        }
    }
}
