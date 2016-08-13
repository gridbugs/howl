use game::update::UpdateSummary;

pub enum Control {
    Action(UpdateSummary),
    Quit,
}
