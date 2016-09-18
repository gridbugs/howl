use game::UpdateSummary;

pub enum MetaAction {
    Update(UpdateSummary),
    PassTurn,
    NotActor,
    Quit,
}
