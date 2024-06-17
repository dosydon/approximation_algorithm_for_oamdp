use mdp::spelling::Letter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellingCommunicationAction {
    Announce(Letter),
    None,
}
