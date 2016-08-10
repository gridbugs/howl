use game::update::UpdateProgram;

pub enum Control {
    Action(UpdateProgram),
    Quit,
}
