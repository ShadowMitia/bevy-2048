

#[cfg(test)]
mod game_test {
    use bevy_2048::*;

    #[test]
    fn setup_grid() {
        let grid = Grid::new();
        assert_eq!(grid.data.len(), 16);
    }

    #[test]
    fn add_random_tiles_to_grid() {
        let mut grid = Grid::new();

        let count_empty_tiles = |grid: &Grid| {
            (0..grid.data.len())
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
