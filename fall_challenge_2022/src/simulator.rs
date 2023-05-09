#[derive(Debug, Clone, Copy, Default)]
struct GameState {
    my_matter: u32,
    enemy_matter: u32,
}

impl GameState {
    fn update_from_stdin(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.my_matter = parse_input!(inputs[0], u32);
        self.enemy_matter = parse_input!(inputs[1], u32);
    }

    fn update() {
        // resolve BUILD
        // MOVE & SPAWN
        // Remove colliding robots

        // Mark tiles

        // Recyclers reduce scraps of tiles
        // Tiles with 0 scraps -> Grass -> Remove units and structures
        // Currency update
        // Check game end
    }

    /// Checks if the game should end
    /// Reasons to end
    /// - a player no longer controls single tile
    /// - 20 turns have passed without any tile changing scraps or owner
    /// - 200 have concluded
    fn check_game_end() {
        // check end
        // check winner?
    }

    /// Player that controls the most tiles
    fn check_winning_player() {}
}
