// A block named by a display definition gets Roast's block entity.
fn roast_entity(game: &mut crate::game::Game<'_>, block: &str, entity: &mut Option<(String, Compound)>) {
    if entity.is_some() {
        return;
    }
    let Some(defs) = roast::defs(|| roast::FILE_TYPES.iter().flat_map(|t| game.native_recipes_of_type(t).iter().map(|r| (r.id.clone(), (*r.json).clone()))).collect()) else {
        return;
    };
    if defs.display(block).is_some() {
        *entity = Some((roast::ENTITY_ID.to_owned(), Compound::new()));
    }
}
