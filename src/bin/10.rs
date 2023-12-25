use advent_of_code::helpers::matrix::{Cell, Direction, Matrix, CARDINALS};
use hashbrown::{HashMap, HashSet};
use once_cell::sync::Lazy;

advent_of_code::solution!(10);

type Pipe = Vec<(Direction, Direction)>;

static PIPES: Lazy<HashMap<char, Pipe>> = Lazy::new(|| {
    HashMap::from_iter([
        (
            '|',
            vec![(Direction::N, Direction::N), (Direction::S, Direction::S)],
        ),
        (
            '-',
            vec![(Direction::E, Direction::E), (Direction::W, Direction::W)],
        ),
        (
            'L',
            vec![(Direction::S, Direction::E), (Direction::W, Direction::N)],
        ),
        (
            'J',
            vec![(Direction::S, Direction::W), (Direction::E, Direction::N)],
        ),
        (
            '7',
            vec![(Direction::E, Direction::S), (Direction::N, Direction::W)],
        ),
        (
            'F',
            vec![(Direction::N, Direction::E), (Direction::W, Direction::S)],
        ),
        ('.', vec![]),
    ])
});

fn find_loop(matrix: &mut Matrix) -> Option<Vec<Cell>> {
    let items: Vec<Cell> = matrix.items().collect();

    items
        .into_iter()
        .find(|c| c.val == 'S')
        .and_then(|start_cell| {
            CARDINALS
                .iter()
                .find_map(|dir| try_loop_from_start(matrix, start_cell, *dir))
        })
}

fn try_loop_from_start(
    matrix: &mut Matrix,
    start: Cell,
    start_dir: Direction,
) -> Option<Vec<Cell>> {
    let mut visited = vec![];

    let mut current_dir = start_dir;
    let mut current_cell = matrix.neighbour(&start, &start_dir);

    loop {
        match current_cell {
            Some(cell) => {
                visited.push(cell);

                if cell.val == 'S' {
                    let from = current_dir;
                    let to = start_dir;

                    let pipe_type = PIPES
                        .iter()
                        .find(|pipe| pipe.1.iter().any(|p| *p == (from, to)))
                        .map(|pipe| *pipe.0)
                        .unwrap();

                    let c = matrix.get_mut(cell.point.row, cell.point.col).unwrap();
                    *c = pipe_type;

                    break;
                }

                let next = PIPES
                    .get(&cell.val)
                    .and_then(|pipe| resolve_direction(pipe, &current_dir))
                    .map(|next_dir| (next_dir, matrix.neighbour(&cell, &next_dir)));

                match next {
                    Some((dir, cell)) => {
                        current_dir = dir;
                        current_cell = cell;
                    }
                    None => return None,
                }
            }
            None => return None,
        }
    }

    Some(visited)
}

fn resolve_direction(pipe: &Pipe, dir: &Direction) -> Option<Direction> {
    pipe.iter().find(|y| y.0 == *dir).map(|y| y.1)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut matrix = Matrix::from(input);
    find_loop(&mut matrix).map(|pipe| pipe.len() / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
