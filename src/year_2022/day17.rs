use anyhow::{anyhow, Result};
use std::collections::HashMap;

use std::time::{Instant};


pub fn run(input: Vec<String>) {
    let start = Instant::now();
    let _part1 = solve_part1(&input);
    
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

    let start = Instant::now();
    let _part2 = solve_part2(&input);
    println!("Time elapsed in part2 is: {:?}", start.elapsed());

}

fn solve_part1(input: &Vec<String>) -> Result<()> {
    let streams = obtain_streams(input)?;
    let mut block_cycle = Pieces::PIECES_ORDER.iter().copied().enumerate().cycle();
    let mut stream_cycle = streams.iter().copied().enumerate().cycle();

    const NUMBER_OF_ROCKS:usize = 2022;
    let height = simulate_up_to(NUMBER_OF_ROCKS, &mut stream_cycle, &mut block_cycle);
    println!("the height of {} rocks is {}", NUMBER_OF_ROCKS, height);

    Ok(())
}

fn solve_part2(input: &Vec<String>) -> Result<()> {
    let streams = obtain_streams(input)?;
    let mut block_cycle = Pieces::PIECES_ORDER.iter().copied().enumerate().cycle();
    let mut stream_cycle = streams.iter().copied().enumerate().cycle();

    const NUMBER_OF_ROCKS:usize = 1_000_000_000_000;
    let height = simulate_up_to(NUMBER_OF_ROCKS, &mut stream_cycle, &mut block_cycle);
    println!("the height of {} rocks is {}", NUMBER_OF_ROCKS, height);

    Ok(())
}

fn obtain_streams(input: &Vec<String>) -> Result<Vec<AirJet>, anyhow::Error> {
    let streams = &input[0]
        .trim()
        .chars()
        .map(|ch| match ch {
            '<' => Ok(AirJet::Left),
            '>' => Ok(AirJet::Right),
            _ => Err(anyhow!("failed to read jet stream - unknown char: {}", ch)),
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(streams.clone())
}

#[derive(Debug, Clone, Copy)]
enum Pieces {
    Horizontal,
    Cross,
    Lshape,
    Vertical,
    Square,
}

impl Pieces {
    const PIECES_ORDER: &'static [Pieces] = &[
        Pieces::Horizontal,
        Pieces::Cross,
        Pieces::Lshape,
        Pieces::Vertical,
        Pieces::Square,
    ];

    /// Returns an iterator with the coordinates of the piece
    fn pieces(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        use Pieces::*;

        match self {
            Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Cross => vec![(x, y + 1),(x + 1, y), (x + 1, y + 1),
                          (x + 2, y + 1),(x + 1, y + 2),
            ],
            Lshape => vec![(x, y),(x + 1, y),(x + 2, y),
                           (x + 2, y + 1),(x + 2, y + 2),
            ],
            Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
        .into_iter()
    }

    /// Places the piece in the board (map) by setting those coordinates to true
    fn place(&self, (x, y): (usize, usize), map: &mut [Vec<bool>]) {
        self.pieces((x, y)).for_each(|(px, py)| {
            map[py][px] = true;
        });
    }

    /// Return true if any of the matches any already placed piece
    fn collides(&self, (x, y): (usize, usize), map: &[Vec<bool>]) -> bool {
        self.pieces((x, y)).any(|(px, py)| map[py][px])
    }

    /// Return width of the pieces
    fn width(&self) -> usize {
        use Pieces::*;
        match self {
            Horizontal => 4,
            Cross => 3,
            Lshape => 3,
            Vertical => 1,
            Square => 2,
        }
    }

    /// Return height of the pieces
    fn height(&self) -> usize {
        use Pieces::*;

        match self {
            Horizontal => 1,
            Cross => 3,
            Lshape => 3,
            Vertical => 4,
            Square => 2,
        }
    }

    /// Moves the piece horizonally based in the incoming stream, unless it would collide
    fn move_horizontally(&self, (x, y): (usize, usize), pattern: AirJet, map: &[Vec<bool>]) -> usize {
        let width = self.width();

        use AirJet::*;
        let new_x = match pattern {
            Left => x.saturating_sub(1),
            Right => (x + 1).clamp(0, 7 - width),
        };

        if self.collides((new_x, y), map) {
            x
        } else {
            new_x
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AirJet {
    Left,
    Right,
}

/// Moves the piece based in the incoming stream, if it would not collide
/// After that, moves the piece down
fn drop_piece_until_placed(
    blocks: &mut impl Iterator<Item = (usize, Pieces)>,
    streams: &mut impl Iterator<Item = (usize, AirJet)>,
    map: &mut [Vec<bool>],
    drop_height: usize,
) -> (usize, usize, usize) {
    let mut pos = (2, drop_height + 3);
    let (block_idx, block) = blocks.next().expect("infinite iterator");

    let jet_idx = loop {
        let (jet_idx, jet_pattern) = streams.next().expect("infinite iterator");
        pos.0 = block.move_horizontally(pos, jet_pattern, map);

        // move vertically
        if block.collides((pos.0, pos.1 - 1), map) {
            block.place(pos, map);
            break jet_idx;
        } else {
            pos.1 -= 1;
        }
    };

    (block_idx, jet_idx, drop_height.max(pos.1 + block.height()))
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct State([usize; 7], usize, usize);

impl State {
    // Creates a cache with the ceiling of the map
    fn ceiling_map(map: &[Vec<bool>], height: usize) -> [usize; 7] {
        let mut result = [0; 7];
        for (idx, h) in (0..7)
            .map(|x| {
                let mut height = height;
                let mut count = 0;
                while !map[height][x] {
                    height -= 1;
                    count += 1;
                }

                count
            })
            .enumerate()
        {
            result[idx] = h;
        }

        result
    }
}

fn simulate_up_to(
    target: usize,
    mut stream_cycle: &mut impl Iterator<Item = (usize, AirJet)>,
    mut block_cycle: &mut impl Iterator<Item = (usize, Pieces)>,
) -> usize {
    let upper_bound = 500000;
    let mut map = vec![vec![false; 7]; upper_bound * 4];
    (0..7).for_each(|x| map[0][x] = true);
    let mut cache: HashMap<State, (usize, usize)> = HashMap::new();

    let mut height = 1;
    for block_no in 0..upper_bound {
        let (block_idx, jet_idx, height_now) =
            drop_piece_until_placed(&mut block_cycle, &mut stream_cycle, &mut map, height);
        height = height_now;

        let ceiling = State::ceiling_map(&map, height);
        let state = State(ceiling, block_idx, jet_idx);

        // If the state is saved in the cache, we add the current height multiplied by the cocient of the target / how many blocks
        // and then the remainder
        if cache.contains_key(&state) {
            let (blocks_placed_before, height_before) = cache.get(&state).copied().unwrap();
            let how_many_blocks = block_no + 1 - blocks_placed_before;
            let height_diff = height - height_before;
            let repeats = (target - (block_no + 1)) / how_many_blocks;
            let remainder_count = (target - (block_no + 1)) - (repeats * how_many_blocks);
            let mut total_height = height_diff * repeats;

            (0..remainder_count).for_each(|_| {
                let (_, _, height_now) =
                    drop_piece_until_placed(&mut block_cycle, &mut stream_cycle, &mut map, height);
                height = height_now;
            });

            total_height += height;
            return total_height - 1;
        } else {
            cache.insert(state, (block_no + 1, height));
        }
    }

    height
}