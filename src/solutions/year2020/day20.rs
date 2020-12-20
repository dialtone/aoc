use super::*;

use std::collections::*;

type Input = String;
type Parsed = String;

type Tile = Vec<String>;

pub fn part1(input: &str) -> usize {
    // tiles outside have an outside border that doesn't line up with anything
    // so tiles at the corner have 2 borders that don't line up (4 when also flipped).
    // and multiply them.
    let tiles_input = input.split("\n\n");
    let mut tiles = HashMap::new();

    for tile_and_id in tiles_input {
        if tile_and_id == "" {
            continue;
        }
        let mut n = tile_and_id.lines();
        let tile_id = n.next().unwrap();
        assert!(tile_id.starts_with("Tile "));
        tiles.insert(&tile_id[5..tile_id.len() - 1], n.collect::<Vec<&str>>());
    }

    // Any tile_id with 2 edges with only 1 shared_edge is a corner.
    let mut shared_edges = HashMap::new();
    let mut tile_edges = HashMap::new();

    for (tile_id, tile) in tiles {
        let top_edge = tile[0].to_string();
        let left_edge = tile.iter().map(|&row| row.chars().next().unwrap()).join("");
        let right_edge = tile
            .iter()
            .map(|&row| row.chars().last().unwrap())
            .collect::<String>();
        let bottom_edge = tile.last().unwrap().to_string();

        for edge in [top_edge, right_edge, bottom_edge, left_edge].iter() {
            let flipped_edge = edge.chars().rev().collect::<String>();

            let entry = tile_edges.entry(tile_id).or_insert(Vec::new());
            entry.push(edge.clone());
            entry.push(flipped_edge.clone());

            shared_edges
                .entry(edge.clone())
                .or_insert(HashSet::new())
                .insert(tile_id);
            shared_edges
                .entry(flipped_edge.clone())
                .or_insert(HashSet::new())
                .insert(tile_id);
        }
    }

    let mut corners = vec![];
    for (tile_id, edges) in tile_edges {
        // println!("{} => {:?}", tile_id, edges);
        let tile_shared_edges = edges
            .iter()
            .map(|edge| shared_edges.get(edge).unwrap())
            .collect::<Vec<_>>();
        // println!("\tShared => {:?}", tile_shared_edges);

        let mut tile_set = HashSet::new();
        tile_set.insert(tile_id);

        let shared_sets_sizes = tile_shared_edges
            .iter()
            .map(|&tile_shared_edge_set| {
                tile_shared_edge_set.symmetric_difference(&tile_set).count()
            })
            .collect::<Vec<_>>();
        // println!("\tShared Set Size => {:?}", shared_sets_sizes);
        let zero_shared = shared_sets_sizes.iter().filter(|&&x| x == 0).count();
        if zero_shared == 4 {
            // 2 right, 2 flipped missing shares make these corners
            corners.push(tile_id.parse::<usize>().unwrap());
        }
    }

    // println!("corners => {:?}", corners);
    corners.iter().product()
}

fn rotated(tile: &Tile) -> Tile {
    let mut rotated = vec![vec!['X'; tile[0].len()]; tile.len()];
    for (i, row) in tile.iter().enumerate() {
        for (j, col) in row.chars().enumerate() {
            rotated[j][row.len() - 1 - i] = col;
        }
    }

    rotated.iter().map(|rr| rr.iter().collect()).collect()
}

fn flipped(tile: &Tile) -> Tile {
    tile.iter().map(|row| row.chars().rev().collect()).collect()
}

fn rotations(tile: &Tile) -> Vec<Tile> {
    let mut rots = vec![tile.clone()];
    let mut curr_tile = tile.clone();
    for _rot in 0..4 {
        let rot = rotated(&curr_tile);
        let fliprot = flipped(&rot);
        curr_tile = rot.clone();
        rots.push(rot);
        rots.push(fliprot);
    }
    rots
}

fn remove_outer_layer(tile: Tile) -> Tile {
    let tile_len = tile.len();
    let row_len = tile[0].len();
    let tile: Tile = tile
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            if i == 0 || i == tile_len - 1 {
                "".to_owned()
            } else {
                row[1..tile_len - 1].to_owned()
            }
        })
        .filter(|x| x.len() != 0)
        .collect();
    assert_eq!(tile.len(), tile_len - 2);
    assert_eq!(tile[0].len(), row_len - 2);
    tile
}

fn join_all_tiles(tiles: &Vec<Vec<Tile>>) -> Tile {
    let mut joined: Vec<Vec<String>> =
        vec![vec!["".to_owned(); tiles[0].len()]; tiles[0][0].len() * tiles.len()];

    let tile_rows_len = tiles[0][0].len();
    for (i, row_of_tiles) in tiles.iter().enumerate() {
        for (j, col_of_tiles) in row_of_tiles.iter().enumerate() {
            for (k, row_in_tile) in col_of_tiles.iter().enumerate() {
                joined[i * tile_rows_len + k][j] = row_in_tile.clone();
            }
        }
    }

    joined.into_iter().map(|row| row.join("")).collect()
}

pub fn part2(input: &Parsed) -> usize {
    // 10x10 squares, rotate 4 times, flip each time to build the
    // edges, to compose the map just dfs through the tiles looking for those with
    // shared borders rotate them until they fit and apply and go forward, if you can't find
    // an option go back and switch choice by backtracking

    5
}

pub fn parse(s: &Input) -> &Parsed {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_outer() {
        let m: Tile = vec![
            "....".to_owned(),
            ".##.".to_owned(),
            ".##.".to_owned(),
            "....".to_owned(),
        ];
        assert_eq!(
            remove_outer_layer(m),
            vec!["##".to_owned(), "##".to_owned()]
        )
    }

    #[test]
    fn test_flip() {
        let m: Tile = vec![
            "#...".to_owned(),
            "#...".to_owned(),
            "#...".to_owned(),
            "#...".to_owned(),
        ];
        let m_flip: Tile = vec![
            "...#".to_owned(),
            "...#".to_owned(),
            "...#".to_owned(),
            "...#".to_owned(),
        ];
        assert_eq!(flipped(&m), m_flip);
    }

    #[test]
    fn test_join_tiles() {
        let m: Tile = vec![
            "#...".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
        ];
        let m_rot1: Tile = vec![
            "...#".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
        ];
        let m_rot2: Tile = vec![
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "...#".to_owned(),
        ];
        let m_rot3: Tile = vec![
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "#...".to_owned(),
        ];
        let all = vec![vec![m, m_rot1], vec![m_rot3, m_rot2]];
        let joined = vec![
            "#......#", "........", "........", "........", "........", "........", "........",
            "#......#",
        ];
        assert_eq!(join_all_tiles(&all), joined);
    }

    #[test]
    fn test_rotate() {
        let m: Tile = vec![
            "#...".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
        ];
        let m_rot1: Tile = vec![
            "...#".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
        ];
        let m_rot2: Tile = vec![
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "...#".to_owned(),
        ];
        let m_rot3: Tile = vec![
            "....".to_owned(),
            "....".to_owned(),
            "....".to_owned(),
            "#...".to_owned(),
        ];
        assert_eq!(rotated(&m), m_rot1);
        assert_eq!(rotated(&m_rot1), m_rot2);
        assert_eq!(rotated(&m_rot2), m_rot3);
    }

    #[test]
    fn test_day20() {
        let test_input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
            .to_owned();
        assert_eq!(part1(&test_input), 20899048083289);
        // assert_eq!(part2(&parse(&test_input)), 5);
    }

    #[test]
    fn day20() {
        let input = get_input(2020, 20).unwrap();
        assert_eq!(part1(&input), 174206308298779);
        // assert_eq!(part2(&parse(&input)), 5);
    }
}
