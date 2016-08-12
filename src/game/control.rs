use game::update::UpdateProgram;
use game::update::UpdateSummary_;

pub enum Control {
    Action(UpdateSummary_),
    Quit,
}
