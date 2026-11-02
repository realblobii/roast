// A display has one item slot, no menu, and no hopper access.
fn roast_container(game: &crate::game::Game<'_>, block: &str, container: &mut Option<ModContainer>) {
    let Some(defs) = roast::defs(|| roast::FILE_TYPES.iter().flat_map(|t| game.native_recipes_of_type(t).iter().map(|r| (r.id.clone(), (*r.json).clone()))).collect()) else {
        return;
    };
    if defs.display(block).is_some() {
        *container = Some(ModContainer { slots: 1, faces: std::array::from_fn(|_| Vec::new()), inputs: Vec::new(), outputs: Vec::new(), menu: None });
    }
}
