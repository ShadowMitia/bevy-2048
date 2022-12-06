use bevy::prelude::*;
use bevy_2048::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("000000").unwrap()))
        .insert_resource(Score(0))
        .insert_resource(Grid::new())
        .add_event::<ScoreEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "ShadowMitia's 2048".to_string(),
                width: WINDOW_SIZE.x,
                height: WINDOW_SIZE.y,
                ..default()
            },
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_startup_system(setup_game)
        .add_system(update_game)
        .add_system(input)
        .add_system(update_score)
        .run();
}
