use std::vec;
use std::collections::HashMap;
use std::time::Instant;

/// Holds information about which tile is in which position.
/// Should be fairly compact and easy to copy.
#[derive(Debug, Clone)]
pub struct GameState {
    board: Vec<Vec<Option<u8>>>
}

/// Creates the default position of tiles, starting with 1 in the top left corner.
impl Default for GameState {
    fn default() -> Self {
        // vector in column-major order
        let x = vec![vec![Some(1), Some(5), Some(9), Some(13)],
            vec![Some(2), Some(6), Some(10), Some(14)],
            vec![Some(3), Some(7), Some(11), Some(15)],
            vec![Some(4), Some(8), Some(12), None]];
        return Self{board : x};
    }
}

/// Generates a human-readable representation of the game state.
impl std::fmt::Display for GameState {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for x in 0..4 {
            for y in 0..4 {
                match self.board[y][x] {
                    Some (x) =>  str.push_str(&format!("| {:>2} ", {x})),
                    None => str.push_str("|    ")
                }                        
            }
            str.push_str("|\n");
        }
        write!(f, "{}", str)
    } 
}

/// Checks whether two game states are the same,.
impl PartialEq for GameState {
    // game states are equal when 2 boards have the exact same numbers at every board location
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] != other.board[i][j] {return false;}
            }
        }

        return true;
    }
}

/// Feel free to ignore this. (but do not remove)
impl Eq for GameState {}

impl GameState {
    /// Updates a position with a new tile.
    pub fn set(&mut self, x: u8, y: u8, tile: Option<u8>) {
        self.board[x as usize][y as usize] = tile; 
    }

    /// Returns the tile at position x,y.
    pub fn get(&self, x: u8, y: u8) -> Option<u8> {
        *self.board.get(x as usize).unwrap().get(y as usize).unwrap()
    }

    /// Returns false if there is a duplicate tile in this game state.
    pub fn all_tiles_unique(&self) -> bool {
        // there are 15 valid numbers; if we have seen number i so far on the board, i is set to 1. None is at loc 0.
        let mut tile_tracker: [i32; 16] = [0; 16];

        for i in 0..4 {
            for j in 0..4 {
                match self.board[i][j] {
                    Some (x) => 
                        if x > 15 || x < 1 {
                            return false; // Invalid Board: Certain numbers out of range.
                        }
                        else if tile_tracker[(x) as usize] >= 1 {return false} // there is a duplicate
                        else {tile_tracker[(x) as usize] = 1;} // this is a number we haven't seen before; there is not a duplicate so far
                    ,
                    None => {
                        if tile_tracker[0] >= 1 {return false} // there is a duplicate
                        else {tile_tracker[0] = 1;} // this is a number we haven't seen before; there is not a duplicate so far
                    },
                }
            }
        }
        return true;

    }

    // returns the (x, y) location of the empty spot on the board
    fn empty_loc(&mut self) -> (u8, u8) {
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == None { return (i as u8, j as u8) } 
            }
        }
        panic!("Invalid Board: There are no empty positions on the board.")
    }

    /// Swaps the tile from (x1,y1) with the tile from (x2,y2)
    pub fn swap(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        let x1 = x1 as usize; let y1 = y1 as usize;
        let x2 = x2 as usize; let y2 = y2 as usize;

        let tmp = self.board[x1 as usize][y1 as usize];
        self.board[x1][y1] = self.board[x2][y2];
        self.board[x2][y2] = tmp;
    }

    /// Updates the state to reflect the move that was performed. Returns false if the move was
    /// not possible.
    pub fn perform_move(&mut self, m: Move) -> bool {
        let (x, y) = self.empty_loc();
        match m {
            Move::LeftToRight => if  x == 0  { false } else { self.swap(x, y, x - 1, y); true}
            Move::RightToLeft => if  x == 3  { false } else { self.swap(x, y, x + 1, y); true}
            Move::BottomToTop => if  y == 3  { false } else { self.swap(x, y, x, y + 1); true}
            Move::TopToBottom => if  y == 0  { false } else { self.swap(x, y, x, y - 1); true}
        }
    }

    /// Performs a series of moves. Returns the number of moves that were successful.
    pub fn perform_moves(&mut self, moves: &[Move]) -> usize {
        let mut count = 0;
        for i in moves {
            if self.perform_move(*i) {count += 1;}
        }

        return count;
    }

    /// Tries to parse a game state from the provided string.
    /// Returns None if parsing is not possible, or if the parsed game state would contain
    /// duplicate or invalid tiles.
    /// Ignores whitespace.
    pub fn from_str(s: &str) -> Option<Self> {
        let mut matrix = vec![vec![None; 4]; 4]; // Initialize a 4x4 matrix of None

        // Split the input into lines
        let rows: Vec<&str> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();
    
        // Iterate through each row
        if rows.len() != 4 { return None }; // invalid # of rows
        for (row_index, row) in rows.iter().enumerate() {
            let elements: Vec<&str> = row.split('|').map(str::trim).collect();
            if elements.len() != 6 { return None }; // there is not the proper number of '|'s / invalid # of cols
            for col_index in 0..4 {
                let element = elements[col_index + 1]; // Skip the first '|' character
                if element.is_empty() {
                    matrix[col_index][row_index] = None; // Empty slot
                } else {
                    match element.parse::<u8>() {
                        Ok(value) => matrix[col_index][row_index] = Some(value),
                        Err(_) => return None,
                    }
                }
            }
        }    

        let state = GameState{board: matrix};
        // println!("{:?}", state.board);
        // println!("{:?}, Unique: {}", state.board, GameState::all_tiles_unique(&state));
        if GameState::all_tiles_unique(&state) {Some(state)} else {None}
    }
}

/// Finds the minimal number of moves needed to get from one state to the other.
/// Might run forever if there is no path, so use with caution!
pub fn find_shortest_path(from: GameState, to: GameState) -> Vec<Move> {
    let mut paths: Vec::<Vec<Move>> = vec![];
    paths.push(vec![]);

    // states are keys and values are the shortest paths to reach that state
    let mut states_discovered: HashMap<Vec<Vec<Option<u8>>>, bool> = HashMap::new(); 

    for _i in 1..=1000 { 
        let mut new_paths: Vec::<Vec<Move>> = vec![];
        for path in paths {
            for mv in vec![Move::LeftToRight, Move::RightToLeft, Move::TopToBottom, Move::BottomToTop] {
                let mut curr_state = from.clone();
                curr_state.perform_moves(&path);
                let mut new_path = path.clone(); 

                if curr_state.perform_move(mv) {     
                    if states_discovered.contains_key(&curr_state.board) {
                        // ignore this move since we've already reached it once
                    }
                    else {
                        // add this state to our hashmap
                        states_discovered.insert(curr_state.board.clone(), true);

                        // add this path to our new paths
                        new_path.push(mv); 
                        new_paths.push(new_path.clone()); 
                    }

                    
                }

                if curr_state == to { return new_path; }
            };
        };

        paths = new_paths;
        // println!("Paths for i = {}\n{:?}", i, paths);
    };  

    panic!("Did not find any valid path of any valid length");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum Move {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_state() {
        let state = GameState::default();
        assert_eq!(state.get(0, 0), Some(1));
        assert_eq!(state.get(1, 0), Some(2));
        assert_eq!(state.get(2, 0), Some(3));
        assert_eq!(state.get(3, 0), Some(4));
        assert_eq!(state.get(0, 1), Some(5));
        assert_eq!(state.get(1, 1), Some(6));
        assert_eq!(state.get(2, 1), Some(7));
        assert_eq!(state.get(3, 1), Some(8));
        assert_eq!(state.get(0, 2), Some(9));
        assert_eq!(state.get(1, 2), Some(10));
        assert_eq!(state.get(2, 2), Some(11));
        assert_eq!(state.get(3, 2), Some(12));
        assert_eq!(state.get(0, 3), Some(13));
        assert_eq!(state.get(1, 3), Some(14));
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
    }

    #[test]
    fn test_set_game_state() {
        let mut state = GameState::default();
        state.set(0, 2, Some(3));
        assert_eq!(state.get(0, 2), Some(3));
        // TODO: add more tests
    }

    const DEFAULT_STATE_STR: &'static str = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 | 15 |    |
";

    #[test]
    fn test_display_game_state() {
        let state = GameState::default();
        assert_eq!(DEFAULT_STATE_STR, format!("{state}"));

        // TODO: add more tests
    }

    #[test]
    fn test_validate_game_state() {
        let mut state = GameState::default();
        assert!(state.all_tiles_unique());
        state.set(3, 0, Some(1));
        assert!(!state.all_tiles_unique());
        state.set(0, 0, Some(4));
        assert!(state.all_tiles_unique());

        // TODO: add more tests
    }

    #[test]
    fn test_swap() {
        let mut state = GameState::default();
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
        state.swap(2, 3, 3, 3);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(2, 3), None);
        assert_eq!(state.get(3, 3), Some(15));

        state.swap(0, 0, 2, 2);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 0), Some(11));

    }

    #[test]
    fn test_perform_move() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::RightToLeft));
        assert!(!state.perform_move(Move::BottomToTop));
        assert!(state.perform_move(Move::TopToBottom));
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(3, 3), Some(12));
        assert_eq!(state.get(3, 2), None);
        assert!(state.perform_move(Move::LeftToRight));
        assert_eq!(state.get(3, 2), Some(11));
        assert_eq!(state.get(2, 2), None);
        state.perform_move(Move::LeftToRight);
        assert_eq!(state.get(2, 2), Some(10));
        assert_eq!(state.get(1, 2), None);
    }

    #[test]
    fn test_game_state_equality() {
        {
            let mut state = GameState::default();
            assert!(!state.perform_move(Move::BottomToTop));
            assert_eq!(state, GameState::default());
            assert!(state.perform_move(Move::TopToBottom));
            let mut state_2 = GameState::default();
            state_2.set(3, 3, Some(12));
            state_2.set(3, 2, None);
            assert_eq!(state, state_2);
        }
  
        {
            let mut state = GameState::default();
            assert!(state.perform_move(Move::LeftToRight));
            assert!(state.perform_move(Move::RightToLeft));
            let state_2 = GameState::default();
            assert_eq!(state, state_2);
        }
    }

    #[test]
    fn test_perform_moves() {
        { 
            let mut state = GameState::default();
            assert_eq!(
                state.perform_moves(&[Move::RightToLeft, Move::BottomToTop, Move::TopToBottom]),
                1
            );

            let mut state = GameState::default();
            assert_eq!(
                state.perform_moves(&[Move::TopToBottom, Move::TopToBottom, Move::TopToBottom]),
                3
            );
            let expected = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
            assert_eq!(expected, format!("{state}"));

        }

        { 
            let mut state = GameState::default();
            assert_eq!(
                state.perform_moves(&[Move::LeftToRight, Move::LeftToRight, Move::LeftToRight]),
                3
            );
            let expected = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
|    | 13 | 14 | 15 |
";
            assert_eq!(expected, format!("{state}"));

        }

        { 
            let mut state = GameState::default();
            assert_eq!(
                state.perform_moves(&[Move::LeftToRight, Move::RightToLeft, Move::LeftToRight]),
                3
            );
            let expected = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 |    | 15 |
";
            assert_eq!(expected, format!("{state}"));

        }
    }

    #[test]
    fn test_parse_state() {
        assert_eq!(
            GameState::from_str(DEFAULT_STATE_STR).unwrap(),
            GameState::default()
        );

        let wrong0 = "\
|  1 | 22 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong1 = "\
|  1 |  2 ,  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong2 = "\
|  1 |  2 |  3 |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong3 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
| 13 | 14 | 15 | 12 |
";
        let wrong4 = "\
|  1 |  2 |  3 |    | 1 |
|  5 |  6 |  7 |  4 | 1 |
|  9 | 10 | 11 |  8 | 1 |
| 13 | 14 | 15 | 12 | 1 |
";
        let wrong5 = "\
|  1 |  2 |  3 |    |
|  5 |  2 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        assert!(GameState::from_str(wrong0).is_none());
        assert!(GameState::from_str(wrong1).is_none());
        assert!(GameState::from_str(wrong2).is_none());
        assert!(GameState::from_str(wrong3).is_none());
        assert!(GameState::from_str(wrong4).is_none());
        assert!(GameState::from_str(wrong5).is_none());

        // TODO: add more tests
    }

    #[test]
    fn test_find_shortest_path() {
        // Test 1: simple path up the board
        {
            let expected_moves = [Move::TopToBottom, Move::TopToBottom, Move::TopToBottom];
            let mut state = GameState::default();
            assert_eq!(state.perform_moves(&expected_moves), 3);

            let actual_moves = find_shortest_path(GameState::default(), state);
            assert_eq!(actual_moves.len(), 3);
            assert_eq!(actual_moves, expected_moves);
        }

        // Test 2: rotation
        {
            let expected_moves = [Move::TopToBottom, Move::LeftToRight, Move::BottomToTop];
            let mut state = GameState::default();
            assert_eq!(state.perform_moves(&expected_moves), 3);

            let actual_moves = find_shortest_path(GameState::default(), state);
            assert_eq!(actual_moves.len(), 3);
            assert_eq!(actual_moves, expected_moves);
        }

        // Test 3: longer path
        {
            let expected_moves = [Move::TopToBottom, Move::LeftToRight, Move::LeftToRight, Move::LeftToRight, Move::TopToBottom];
            let mut state = GameState::default();
            assert_eq!(state.perform_moves(&expected_moves), 5);

            let actual_moves = find_shortest_path(GameState::default(), state);
            assert_eq!(actual_moves.len(), 5);
            assert_eq!(actual_moves, expected_moves);
        }

    }
}
