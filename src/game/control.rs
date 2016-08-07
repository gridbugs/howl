use game::update::UpdateProgramFn;

pub enum Control {
    Action(UpdateProgramFn),
    Quit,
}
