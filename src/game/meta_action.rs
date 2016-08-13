use game::UpdateSummary;

pub enum MetaAction {
    Update(UpdateSummary),
    Quit,
}
