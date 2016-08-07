use game::update::monad::UpdateMonad;

pub enum Control {
    Action(UpdateMonad<()>),
    Quit,
}
