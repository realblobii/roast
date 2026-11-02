// Displays tick, to keep the shown item in their saved data for clients to draw.
fn roast_ticks(game: &crate::game::Game<'_>, block: &str, ticks: &mut bool) {
    if let Some(defs) = roast::defs(|| roast::FILE_TYPES.iter().flat_map(|t| game.native_recipes_of_type(t).iter().map(|r| (r.id.clone(), (*r.json).clone()))).collect()) {
        if defs.display(block).is_some() {
            *ticks = true;
        }
    }
}
