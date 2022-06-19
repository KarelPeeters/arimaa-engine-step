#[macro_export]
macro_rules! take_actions {
    ($($action: expr),* $(,)?) => {{
        take_actions![GameState::initial() => $($action),*]
    }};
    ($game_state: expr => $($action: expr),*) => {{
        let mut game_state = $game_state;
        $({
            let action = <$crate::Action as std::str::FromStr>::from_str(stringify!($action)).unwrap();
            game_state = game_state.take_action(&action);
        })*
        game_state
    }};
}

#[macro_export]
macro_rules! board {
    ($board: literal) => {
        <$crate::GameState as std::str::FromStr>::from_str($board).unwrap()
    };
}

#[macro_export]
macro_rules! action {
    ($action: ident) => {
        <$crate::Action as std::str::FromStr>::from_str(stringify!($action)).unwrap()
    };
}
