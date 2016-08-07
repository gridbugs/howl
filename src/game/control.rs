use game::update::UpdateMonad;

pub enum Control {
    Action(UpdateMonad<()>),
    Quit,
}
