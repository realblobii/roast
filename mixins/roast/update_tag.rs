// Clients get the saved data (what is `shown`).
fn roast_update_tag(entity: &ModBlockEntity, tag: &mut Option<Compound>) {
    if entity.id == roast::ENTITY_ID {
        *tag = Some(entity.nbt.clone());
    }
}
