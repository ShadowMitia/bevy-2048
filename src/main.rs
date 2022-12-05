use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

use bevy::{prelude::*, winit::WinitSettings};
use rand::{seq::SliceRandom, thread_rng};

const CELL_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Debug)]
struct Cell {
    index: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Array2D {
    data: [u32; 16],
}

impl Array2D {
    fn new() -> Self {
        Self { data: [0; 16] }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl PartialEq<[u32; 16]> for Array2D {
    fn eq(&self, other: &[u32; 16]) -> bool {
        &self.data == other
    }
}

impl From<&[u32; 16]> for Array2D {
    fn from(slice: &[u32; 16]) -> Self {
        Self { data: *slice }
    }
}

impl From<[u32; 16]> for Array2D {
    fn from(slice: [u32; 16]) -> Self {
        Self { data: slice }
    }
}

impl Index<usize> for Array2D {
    type Output = [u32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        self.data[index * 4..(index + 1) * 4]
            .try_into()
            .expect("Invalid index")
    }
}

impl IndexMut<usize> for Array2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let d = self.data[index * 4..(index + 1) * 4]
            .as_mut()
            .try_into()
            .expect("Invalid index");
        d
    }
}

#[derive(Resource, Debug, Clone, Copy)]
struct Grid {
    data: Array2D,
}

impl Grid {
    fn new() -> Self {
        Grid {
            data: Array2D::new(),
        }
    }

    fn add_random_tile(&mut self) -> bool {
        let mut rng = thread_rng();

        let mut empty_indices: Vec<usize> = (0..self.data.len())
            .filter(|&index| self.data[index / 4][index.rem_euclid(4)] == 0)
            .collect();

        if empty_indices.is_empty() {
            false
        } else {
            empty_indices.shuffle(&mut rng);
            let index = empty_indices[0];
            // TODO: check probabilities in original game
            self.data[index / 4][index.rem_euclid(4)] = if rand::random::<bool>() { 2 } else { 4 };
            true
        }
    }

    fn move_left(&mut self) -> u32 {
        let mut score = 0;
        for j in 0..4 {
            for i in 1..4 {
                for k in 0..i {
                    if self.data[j][k] == 0 {
                        (self.data[j][i], (self.data[j][k])) = (self.data[j][k], (self.data[j][i]));
                    } else if self.data[j][i] == self.data[j][k] {
                        self.data[j][k] *= 2;
                        score += self.data[j][k];
                        self.data[j][i] = 0;
                    }
                }
            }
        }
        score
    }

    fn move_right(&mut self) -> u32 {
        let mut score = 0;
        for j in 0..4 {
            for i in 0..3 {
                for k in i + 1..4 {
                    if self.data[j][k] == 0 {
                        (self.data[j][i], (self.data[j][k])) = (self.data[j][k], (self.data[j][i]));
                    } else if self.data[j][i] == self.data[j][k] {
                        self.data[j][k] *= 2;
                        score += self.data[j][k];
                        self.data[j][i] = 0;
                    }
                }
            }
        }
        score
    }

    fn move_down(&mut self) -> u32 {
        let mut score = 0;
        for _ in 0..4 {
            for j in 1..4 {
                for i in 0..4 {
                    for k in 0..j {
                        if self.data[k][i] == 0 {
                            (self.data[j][i], (self.data[k][i])) =
                                (self.data[k][i], (self.data[j][i]));
                        } else if self.data[k][i] == self.data[j][i] {
                            self.data[k][i] *= 2;
                            score += self.data[k][i];
                            self.data[j][i] = 0;
                        }
                    }
                }
            }
        }
        score
    }

    fn move_up(&mut self) -> u32 {
        let mut score = 0;
        for j in 0..3 {
            for i in 0..4 {
                for k in j + 1..4 {
                    if self.data[k][i] == 0 {
                        (self.data[j][i], (self.data[k][i])) = (self.data[k][i], (self.data[j][i]));
                    } else if self.data[k][i] == self.data[j][i] {
                        self.data[k][i] *= 2;
                        score += self.data[k][i];
                        self.data[j][i] = 0;
                    }
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod game_test {
    use super::*;

    #[test]
    fn setup_grid() {
        let grid = Grid::new();
        assert_eq!(grid.data.data.len(), 16);
    }

    #[test]
    fn add_random_tiles_to_grid() {
        let mut grid = Grid::new();

        let count_empty_tiles = |grid: &Grid| {
            (0..grid.data.data.len())
                .filter(|&index| grid.data[index / 4][index.rem_euclid(4)] == 0)
                .count()
        };

        assert_eq!(count_empty_tiles(&grid), 16);

        grid.add_random_tile();

        assert_eq!(count_empty_tiles(&grid), 15);

        grid.add_random_tile();

        assert_eq!(count_empty_tiles(&grid), 14);
    }

    #[test]
    fn move_grid_left() {
        let mut grid = Grid::new();

        grid.data.data = [0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0];

        // Move grid left, aligns cells
        grid.move_left();

        let new_grid = [0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(grid.data.data, new_grid);

        // Move grid left again, cells don't move from previous step
        grid.move_left();

        assert_eq!(grid.data.data, new_grid);
    }

    #[test]
    fn move_grid_right() {
        let mut grid = Grid::new();

        grid.data.data = [0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0];

        // Move grid right, aligns cells
        grid.move_right();

        let new_grid = [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0];

        assert_eq!(grid.data.data, new_grid);

        // Move grid right again, cells don't move from previous step
        grid.move_right();

        assert_eq!(grid.data.data, new_grid);
    }

    #[test]
    fn move_grid_down() {
        let mut grid = Grid::new();

        grid.data.data = [0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0];

        // Move grid down, aligns cells
        grid.move_down();

        let new_grid = [0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(grid.data.data, new_grid);

        // Move grid down again, cells don't move from previous step
        grid.move_down();

        assert_eq!(grid.data.data, new_grid);
    }

    #[test]
    fn move_grid_up() {
        let mut grid = Grid::new();

        grid.data.data = [0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0];

        // Move grid up, aligns cells
        grid.move_up();

        let new_grid = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0];

        assert_eq!(grid.data.data, new_grid);

        // Move grid up again, cells don't move from previous step
        grid.move_up();

        assert_eq!(grid.data.data, new_grid);
    }

    #[test]
    fn move_grid_up_with_adds() {
        let mut grid = Grid::new();

        grid.data.data = [0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0];

        // Move grid up, aligns cells
        grid.move_up();

        grid.data.data[10] = 4;

        let new_grid = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 2, 2, 0];

        assert_eq!(new_grid[10], 4);
        assert_eq!(grid.data.data[10], 4);
        assert_eq!(grid.data.data, new_grid);

        dbg!(&grid);

        // Move grid up again, cells don't move from previous step
        grid.move_up();

        dbg!(&grid);

        assert_eq!(new_grid[10], 4);
        assert_eq!(grid.data.data[10], 4);
        assert_eq!(grid.data.data, new_grid);
    }
}

struct ScoreEvent(u32);

#[derive(Resource)]
struct Score(u32);

fn score_to_colour(score: u32) -> Color {
    match score {
        0 => Color::hex("cdc1b4").unwrap(),
        2 => Color::hex("eee4da").unwrap(),
        4 => Color::hex("ede0c8").unwrap(),
        8 => Color::hex("f2b179").unwrap(),
        16 => Color::hex("f59563").unwrap(),
        32 => Color::hex("f67c5f").unwrap(),
        64 => Color::hex("f65e3b").unwrap(),
        128 => Color::hex("edcf72").unwrap(),
        256 => Color::hex("edcc61").unwrap(),
        512 => Color::hex("edc850").unwrap(),
        1024 => Color::hex("edc53f").unwrap(),
        2048 => Color::hex("edc22e").unwrap(),
        _ => Color::hex("FF00FF").unwrap(),
    }
}

#[derive(Resource)]
struct GameStyle(TextStyle);

fn setup_game(mut commands: Commands, mut grid: ResMut<Grid>, asset_server: Res<AssetServer>) {
    let cell_bg: Color = score_to_colour(0);
    //    let cell_border: Color = Color::hex("bbada0").unwrap();

    let text_dark: Color = Color::hex("776e65").unwrap();
    // const TEXT_LIGHT: Color = Color::hex("#f9f6f2").unwrap();

    let font = asset_server.load("fonts/Kenney Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: text_dark,
    };

    commands.insert_resource(GameStyle(text_style.clone()));

    // Game starts with two random tiles
    grid.add_random_tile();
    grid.add_random_tile();

    // let root = commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //             justify_content: JustifyContent::SpaceBetween,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         // left vertical fill (border)
    //         parent.spawn(NodeBundle {
    //             style: Style {
    //                 size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
    //                 border: UiRect::all(Val::Px(2.0)),
    //                 ..default()
    //             },
    //             background_color: Color::rgb(0.65, 0.65, 0.65).into(),
    //             ..default()
    //         });
    //     });

    let cell_size = CELL_SIZE; // - Vec2::new(padding / 2.0, padding / 2.0);

    for j in 0..4 {
        for i in 0..4 {
            let index = j * 4 + i;
            let mut transform = {
                let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
                transform.translation.x +=
                    cell_size.x * (i as f32) - WINDOW_SIZE.x / 2.0 + cell_size.x / 2.0;
                transform.translation.y +=
                    cell_size.y * (j as f32) - WINDOW_SIZE.y / 2.0 + cell_size.y / 2.0;
                transform
            };
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color: cell_bg,
                    custom_size: Some(CELL_SIZE),
                    ..Default::default()
                },
                transform,
                ..default()
            },));

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: cell_bg,
                        custom_size: Some(CELL_SIZE),
                        ..Default::default()
                    },
                    transform,
                    ..default()
                },
                Cell { index },
            ));

            transform.translation.z = 1.0;

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section("0", text_style.clone())
                        .with_alignment(TextAlignment::CENTER),
                    transform,
                    ..default()
                },
                Cell { index },
            ));
        }
    }
}

fn update_game(
    mut cell_query: Query<(&mut Sprite, &Cell)>,
    mut text_query: Query<(&mut Text, &Cell)>,
    grid: Res<Grid>,
    text_style: Res<GameStyle>,
) {
    for (mut sprite, cell) in cell_query.iter_mut() {
        let val = grid.data.data[cell.index as usize];
        if val == 0 {
            sprite.color = Color::hex("00000000").unwrap();
        } else {
            sprite.color = score_to_colour(val);
        }
    }
    for (mut text, cell) in text_query.iter_mut() {
        let val = grid.data.data[cell.index as usize];
        if val != 0 {
            *text = Text::from_section(val.to_string(), text_style.0.clone())
                .with_alignment(TextAlignment::CENTER);
        } else {
            *text =
                Text::from_section("", text_style.0.clone()).with_alignment(TextAlignment::CENTER);
        }
    }
}

fn input(input: Res<Input<KeyCode>>, mut grid: ResMut<Grid>, mut score: EventWriter<ScoreEvent>) {
    let old_grid = *grid;
    let mut new_score = 0;
    if input.just_pressed(KeyCode::Left) {
        new_score += grid.move_left();
        if !grid.add_random_tile() {
            println!("Game over!");
        }
    } else if input.just_pressed(KeyCode::Right) {
        new_score += grid.move_right();
        if !grid.add_random_tile() {
            println!("Game over!");
        }
    } else if input.just_pressed(KeyCode::Up) {
        new_score += grid.move_up();
        if !grid.add_random_tile() {
            println!("Game over!");
        }
    } else if input.just_pressed(KeyCode::Down) {
        new_score += grid.move_down();
        if !grid.add_random_tile() {
            println!("Game over!");
        }
    }
    if new_score > 0 {
        score.send(ScoreEvent(new_score));
    }
}

fn update_score(mut score_events: EventReader<ScoreEvent>, mut score: ResMut<Score>) {
    for value in score_events.iter() {
        score.0 += value.0;
        println!("New score: {}", score.0);
    }
}

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
