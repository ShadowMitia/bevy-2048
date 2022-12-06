use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub const CELL_SIZE: Vec2 = Vec2::new(200.0, 200.0);
pub const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Debug)]
pub struct Cell {
    index: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Array2D {
    data: [u32; 16],
}

impl Array2D {
    pub fn new() -> Self {
        Self { data: [0; 16] }
    }

    pub fn new_from(f: &[u32; 16]) -> Self {
        Self { data: *f }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
pub struct Grid {
    pub data: Array2D,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            data: Array2D::new(),
        }
    }

    pub fn add_random_tile(&mut self) -> bool {
        let mut rng = thread_rng();

        let mut empty_indices: Vec<usize> = (0..self.data.len())
            .filter(|&index| self.data[index / 4][index.rem_euclid(4)] == 0)
            .collect();

        if empty_indices.is_empty() {
            false
        } else {
            empty_indices.shuffle(&mut rng);
            let index = empty_indices[0];
            self.data[index / 4][index.rem_euclid(4)] =
                if rand::random::<f32>() < 0.9 { 2 } else { 4 };
            true
        }
    }

    // TODO: check movements
    pub fn move_left(&mut self) -> u32 {
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

    pub fn move_right(&mut self) -> u32 {
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

    pub fn move_down(&mut self) -> u32 {
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

    pub fn move_up(&mut self) -> u32 {
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

pub struct ScoreEvent(pub u32);

#[derive(Resource)]
pub struct Score(pub u32);

pub fn score_to_colour(score: u32) -> Color {
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
pub struct GameStyle(pub TextStyle);

pub fn setup_game(mut commands: Commands, mut grid: ResMut<Grid>, asset_server: Res<AssetServer>) {
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

pub fn update_game(
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

pub fn input(
    input: Res<Input<KeyCode>>,
    mut grid: ResMut<Grid>,
    mut score: EventWriter<ScoreEvent>,
) {
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

pub fn update_score(mut score_events: EventReader<ScoreEvent>, mut score: ResMut<Score>) {
    for value in score_events.iter() {
        score.0 += value.0;
        println!("New score: {}", score.0);
    }
}
