
// use core::mem;
use arc4::Arc4;
use heapless::String;

const BOARD_X: u8 = 8;
const BOARD_Y: u8 = 7;
const BOARD_SIZE_U8: u8 = BOARD_X * BOARD_Y; // BOARD_SIZE is now usize
const BOARD_SIZE: usize = BOARD_SIZE_U8 as usize;

const TRIBIT_WIDTH: u8 = 3; // BIT_WIDTH is now usize
const TRIBIT_ARRAY_SIZE: usize = (BOARD_SIZE * TRIBIT_WIDTH as usize) / 8; // All operands are now usize

//==========================================

#[derive(PartialEq)]
enum State {
    Error,
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    End,
}

impl State {
    fn from(from: u8) -> State {
        match from {
            0 => State::Alpha,
            1 => State::Beta,
            2 => State::Gamma,
            3 => State::Delta,
            4 => State::Epsilon,
            5 => State::End,
            6_u8..=u8::MAX => State::Error
        }
    }

    fn value(&self) -> u8 {
        match self {
            State::Alpha => 0,
            State::Beta => 1,
            State::Gamma => 2,
            State::Delta => 3,
            State::Epsilon => 4,
            State::End => 5,
            State::Error => u8::MAX,
        }
    }

    fn ascii(&self) -> char {
        match self {
            State::Error => '?',
            State::Alpha => 'A',
            State::Beta => 'B',
            State::Gamma => 'G',
            State::Delta => 'D',
            State::Epsilon => 'E',
            State::End => '#',
        }
    }
}

struct RandomStateGenerator {
    prb_key: u32
}

impl RandomStateGenerator {
    fn new() -> Self {
        RandomStateGenerator { prb_key: 123456789 }
    }

    fn generate(&mut self) -> State {
        // Represent 'count' as 4 bytes
        let count_bytes = self.prb_key.to_be_bytes(); // Big endian representation
        let key: [u8; 4] = [
            count_bytes[0], count_bytes[1], count_bytes[2], count_bytes[3],
        ];

        // Create an Arc4 instance with the key
        let mut arc4 = Arc4::with_key(&key);

        // Prepare an output buffer for a single pseudo-random byte
        let mut output: [u8; 1] = [0; 1];

        // Generate the pseudo-random byte
        arc4.prga(&mut output);
        
        let prb = output[0] % State::End.value();

        self.prb_key = self.prb_key.wrapping_add(4321 * prb as u32);

        return State::from(prb);
    }
}


//=========================================

pub struct TribitFields {
    data: [u8; TRIBIT_ARRAY_SIZE],
}

impl TribitFields {
    pub fn new() -> Self {
        TribitFields { data: [0; TRIBIT_ARRAY_SIZE] }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        let bit_index = (index as u8) * TRIBIT_WIDTH;
        let byte_index = bit_index / 8;
        let bit_offset = bit_index % 8;

        let mask = (1 << TRIBIT_WIDTH) - 1;

        // Clear the current 3 bit section
        self.data[byte_index as usize] &= !(mask << bit_offset);
        // Set the new value
        self.data[byte_index as usize] |= (value & mask) << bit_offset;
        // Handle boundary crosssing
        if bit_offset > (8 - TRIBIT_WIDTH) {
            self.data[(byte_index + 1) as usize] &= !(mask >> (8 - bit_offset));
            self.data[(byte_index + 1) as usize] |= (value & mask) >> (8 - bit_offset); 
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        let bit_index = (index as u8) * TRIBIT_WIDTH;
        let byte_index = bit_index / 8;
        let bit_offset = bit_index % 8;

        let mask = (1 << TRIBIT_WIDTH) - 1;

        let mut value = self.data[byte_index as usize] >> bit_offset & mask;

        if bit_offset > (8 - TRIBIT_WIDTH) {
            value |= (self.data[(byte_index + 1) as usize] & (mask >> (8 - bit_offset))) << (8 - bit_offset);
        }

        return value;
    }

    pub fn to_string(&self) -> String<BOARD_SIZE> {
        let mut string: String<BOARD_SIZE> = String::new();
        for i in 0..BOARD_SIZE {
            let char_digit = (b'0' + self.get(i)) as char;
            string.push(char_digit).unwrap();
        }
        return string;
    }
}

struct Board {
    data: TribitFields,
    rsg: RandomStateGenerator
}

impl Board {
    fn new() -> Self {
        Board { 
            data: TribitFields::new(),
            rsg: RandomStateGenerator::new()
        }
    }

    fn randomize(&mut self, rsg: &mut RandomStateGenerator) {
        for i in 0..(BOARD_SIZE){
            let new_value = ((self.data.get(i) + rsg.generate().value()) % State::End.value()) + 1;
            self.data.set(i, new_value);
        }
    }

    fn as_string(&mut self) -> String<BOARD_SIZE>{
        let digit_string = self.data.to_string();
        return digit_string
    }

    fn update(&mut self, island: &mut Island) {
        let x_min: u8 = island.left_bound();
        let x_max: u8 = island.right_bound();

        for column in x_min..=x_max {
            let y_min: u8 = island.column_bottom_bound(column);
            let y_max: u8 = island.column_top_bound(column);
            let y_diff: u8 = (y_max - y_min) + 1;
            self.overwrite_column(column, y_max, y_diff)
        }
    }

    fn for_each(&self, runnable: &dyn Fn(u8, u8, char)) {
        for i in 0..BOARD_SIZE {
            let x = Board::index_to_coord_x(i as u8);
            let y = Board::index_to_coord_y(i as u8);
            let state = self.get(i as u8).ascii();
            runnable(x, y, state);
        }
    }

    fn set(&mut self, index: u8, state: State) {
        self.data.set(index as usize, state.value());
    }

    fn get(&self, index: u8) -> State {
        return State::from(self.data.get(index as usize));
    }

    pub fn coords_to_index(x: u8, y: u8) -> u8 {
        return (y * BOARD_X) + x;
    }

    pub fn index_to_coord_x(i: u8) -> u8 {
        return i % BOARD_X;
    }

    pub fn index_to_coord_y(i: u8) -> u8 {
        return i / BOARD_X;
    }

    fn overwrite_cell(&mut self, x: u8, y: u8, y_diff: u8) {
        if y.checked_sub(y_diff).is_none() {
            let new_state: State = self.rsg.generate();
            self.set(Board::coords_to_index(x, y), new_state);
        } else {
            self.set(Board::coords_to_index(x, y), self.get(Board::coords_to_index(x, y - y_diff)));
        }
    }

    fn overwrite_column(&mut self, x: u8, y_max: u8, y_diff: u8) {
        for y in (0..=y_max).rev() {
            self.overwrite_cell(x, y, y_diff);
        }
    }    
}

struct Island {
    data: TribitFields,
    traverse_index: u8,
    append_index: u8
}

impl Island{
    fn new() -> Self {
        Island { 
            data: TribitFields::new(),
            traverse_index: 0, 
            append_index: 0
        }
    }

    fn set(&mut self, index: u8, value: u8) {
        self.data.set(index as usize, value);
    }

    fn get(&self, index: u8) -> u8 {
        return self.data.get(index as usize);
    }

    fn clear(&mut self) {
        self.append_index = 0;
        self.traverse_index = 0;
        for i in 0..BOARD_SIZE {
            self.set(i as u8, State::Error.value())
        }
    }

    fn add(&mut self, i: u8) {
        for j in 0..BOARD_SIZE {
            if self.get(j as u8) == i { 
                return; 
            }
            if self.get(j as u8) == State::Error.value() { 
                break; 
            }
        }
        self.append_index += 1;
        self.set(self.append_index, i)
    }

    fn complete(&self) -> bool
    {
        return self.get(self.traverse_index) == State::Error.value();
    }
    
    fn horizontal_wrap_check(&self, i: u8, v: u8) -> bool {
        return Board::index_to_coord_x(i) != v;
    }

    fn vertical_wrap_check(&self, i: u8, v: u8) -> bool {
        return Board::index_to_coord_y(i) != v;
    }

    fn right_wrap_check(&self, i: u8) -> bool {
        return self.horizontal_wrap_check(i, 7);
    }

    fn bottom_wrap_check(&self, i: u8) -> bool {
        return self.vertical_wrap_check(i, 6);
    }

    fn left_wrap_check(&self, i: u8) -> bool {
        return self.horizontal_wrap_check(i, 0);
    }

    fn top_wrap_check(&self, i: u8) -> bool {
        return self.vertical_wrap_check(i, 0);
    }

    fn mark_right(&mut self,  i: u8, s: &State, board: &Board) {
        if self.right_wrap_check(i) {
            if board.get(i + 1) == *s {
                self.add(i + 1);
            }
        }
    }

    fn mark_bottom(&mut self, i: u8, s: &State, board: &Board) {
        if self.bottom_wrap_check(i) {
            if board.get(i + 8) == *s {
                self.add(i + 8);
            }
        }
    }

    fn mark_left(&mut self, i: u8, s: &State, board: &Board) {
        if self.left_wrap_check(i) {
            if board.get(i - 1) == *s {
                self.add(i - 1);
            }
        }
    }

    fn mark_top(&mut self, i: u8, s: &State, board: &Board) {
        if self.top_wrap_check(i) {
            if board.get(i - 8) == *s {
                self.add(i - 8);
            }
        }
    }

    fn mark_neighbors(&mut self, board: &Board) {
        let look_index: u8 = self.get(self.traverse_index);
        let current: State = board.get(look_index);

        self.mark_right(look_index, &current, board);
        self.mark_bottom(look_index, &current, board);
        self.mark_left(look_index, &current, board);
        self.mark_top(look_index, &current, board);
    }

    fn update(&mut self, island_root_index: u8, board: &Board) {
        self.clear();
        self.add(island_root_index);

        loop {
            self.mark_neighbors(board);
            self.traverse_index += 1;

            if !self.complete() {
                break;
            }
        }
    }

    fn right_bound(&mut self) -> u8 {
        let mut max: u8 = 0;
        let mut i: u8 = 0;

        while self.get(i) != State::Error.value() {
            let current_x: u8 = Board::index_to_coord_x(self.get(i));
            if current_x > max {
                max = current_x;
            }
            i += 1;
        }

        return max;
    }

    fn left_bound(&mut self) -> u8 {
        let mut min: u8 = BOARD_X;
        let mut i: u8 = 0;

        while self.get(i) != State::Error.value() {
            let current_x: u8 = Board::index_to_coord_x(self.get(i));
            if current_x < min {
                min = current_x;
            }
            i += 1;
        }

        return min;
    }


    fn column_bottom_bound(&mut self, column: u8) -> u8 {
        let mut max: u8 = 0;
        let mut i: u8 = 0;

        while self.get(i) != State::Error.value() {
            if Board::index_to_coord_x(self.get(i)) != column {
                continue;
            }
            let current_y: u8 = Board::index_to_coord_y(self.get(i));
            if current_y > max {
                max = current_y;
            }
            i += 1;
        }

        return max;
    }

    fn column_top_bound(&mut self, column: u8) -> u8 {
        let mut min: u8 = BOARD_Y;
        let mut i: u8 = 0;

        while self.get(i) != State::Error.value() {
            if Board::index_to_coord_x(self.get(i)) != column {
                continue;
            }
            let current_y: u8 = Board::index_to_coord_y(self.get(i));
            if current_y < min {
                min = current_y;
            }
            i += 1;
        }
        
        return min;    
    }

    fn valid(&self) -> bool {
        return self.append_index > 2;
    }

    fn for_each(&self, runnable: &dyn Fn(char)) {
        for i in 0..BOARD_SIZE {
            runnable(self.get(i as u8) as char);
        }
    }
}

struct Isles {
    data: TribitFields
}

impl Isles {
    fn new() -> Self {
        Isles { data: TribitFields::new() }
    }

    fn set(&mut self, index: u8, value: u8) {
        self.data.set(index as usize, value);
    }

    fn get(&self, index: u8) -> u8 {
        return self.data.get(index as usize);
    }
    
    fn clear(&mut self) {
        for i in 0..BOARD_SIZE {
            self.set(i as u8, State::Error.value())
        }
    }

    fn island_at(&self, i: u8) -> bool {
        return self.get(i) != State::Error.value();
    }

    fn apply(&mut self, island: &Island) {
        for i in 0..BOARD_SIZE {
            let index = island.get(i as u8);
            if index == State::Error.value() {
                return;
            }
            self.set(index, State::End.value())
        }
    }

    fn update(&mut self, island: &mut Island, board: &Board) {
        self.clear();
        for i in 0..BOARD_SIZE {
            if self.island_at(i as u8) {
                continue;
            }
            island.update(i as u8, board);
            if island.valid() {
                self.apply(island);
            }
        }
    }

    fn exists(&self) -> bool {
        for i in 0..BOARD_SIZE {
            if self.island_at(i as u8) {
                return true;
            }
        }
        return false;
    }

    fn for_each(&self, runnable: &dyn Fn(u8, u8, char)) {
        for i in 0..BOARD_SIZE {
            let x = Board::index_to_coord_x(i as u8);
            let y = Board::index_to_coord_y(i as u8);
            let state = State::from(self.get(i as u8)).ascii();
            runnable(x, y, state);
        }
    }
}

pub struct Game {
    board: Board,
    island: Island,
    isles: Isles,
    updated: bool,
    game_over: bool
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            isles: Isles::new(),
            island: Island::new(),
            updated: false,
            game_over: false
        }
    }

    pub fn update_board(&mut self) { 
        self.board.update(&mut self.island);
    }

    pub fn select(&mut self, x: u8, y: u8) {
        self.updated = false;
        let i: u8 = Board::coords_to_index(x, y);
        if !self.isles.island_at(i) {
            return;
        }
        self.island.update(i, &self.board);
        self.board.update(&mut self.island);
        self.isles.update(&mut self.island, &self.board);
        self.updated = true;
    }

    pub fn board_as_string(&mut self) -> String<BOARD_SIZE>{
        return self.board.as_string();
    }
}