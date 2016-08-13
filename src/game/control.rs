use game::update::UpdateSummary;

pub enum Control {
    Update(UpdateSummary),
    Quit,
}
