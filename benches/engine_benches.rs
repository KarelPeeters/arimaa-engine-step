use criterion::{Bencher, black_box, Criterion, criterion_group, criterion_main};
use arimaa_engine_step::{GameState, take_actions};

fn bench_valid_actions(b: &mut Bencher) {
    let game_state: GameState = "
            1g
             +-----------------+
            8| h c d m r d c h |
            7|   r r r e r r r |
            6|     x     x     |
            5|                 |
            4| r C             |
            3|     x     x     |
            2| R R R R E R R R |
            1| H   D M R D C H |
             +-----------------+
               a b c d e f g h
           "
        .parse()
        .unwrap();

    let game_state = black_box(game_state);

    b.iter(|| {
        let game_state = game_state.clone();
        let game_state = take_actions!(game_state => b4n);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => a4e);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => b2n);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => f2n);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => d7s);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => e7s);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => f7s);
        let valid_actions = game_state.valid_actions();
        if valid_actions.is_empty() {
            panic!();
        }

        let game_state = take_actions!(game_state => g7s);
        game_state.valid_actions()
    });
}

fn bench_main(c: &mut Criterion) {
    c.bench_function("valid_actions", bench_valid_actions);
}

criterion_group!(benches, bench_main);
criterion_main!(benches);