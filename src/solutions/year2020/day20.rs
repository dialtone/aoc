use super::*;

use regex::Regex;
use std::collections::*;

type Tile = Vec<String>;

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

fn join_all_tiles(tiles: &Vec<Vec<Tile>>, spacer: &str) -> Tile {
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

    joined.into_iter().map(|row| row.join(spacer)).collect()
}

fn printgrid(full_grid: &Vec<Vec<Tile>>) {
    let single_tile_rows = full_grid[0][0].len();
    let single_tile_cols = full_grid[0][0][0].len();
    let side_len = full_grid.len();
    let default = " ".repeat(single_tile_cols);

    for i in 0..side_len * single_tile_rows {
        for j in 0..side_len {
            print!(
                "{}   ",
                full_grid[i / single_tile_rows][j]
                    .get(i % single_tile_rows)
                    .unwrap_or(&default)
            );
        }
        print!("\n");
        if i % single_tile_rows == 0 && i > 0 {
            println!("");
            println!("");
            println!("");
        }
    }
    println!("");
    println!("+++++++");
    println!("");
}

fn next_pos(curr_pos: (usize, usize), side_len: usize) -> (usize, usize) {
    if curr_pos.0 == side_len - 1 && curr_pos.1 == side_len - 1 {
        unreachable!();
    } else if curr_pos.0 == side_len - 1 {
        (0, curr_pos.1 + 1)
    } else {
        (curr_pos.0 + 1, curr_pos.1)
    }
}

// TOP,  RIGHT,  BOTTOM,  LEFT
static D_NEIGHBORS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn top(tile: &Tile) -> String {
    tile[0].to_string()
}
fn right(tile: &Tile) -> String {
    tile.iter()
        .map(|row| row.chars().last().unwrap())
        .collect::<String>()
}
fn bottom(tile: &Tile) -> String {
    tile.last().unwrap().to_string()
}
fn left(tile: &Tile) -> String {
    tile.iter().map(|row| row.chars().next().unwrap()).join("")
}

fn neighbors(pos: (usize, usize)) -> Vec<Option<(usize, usize)>> {
    let mut ns = vec![];
    for &(dx, dy) in D_NEIGHBORS.iter() {
        let newx = pos.0 as isize + dx;
        let newy = pos.1 as isize + dy;
        if newx >= 0 && newy >= 0 {
            ns.push(Some((newx as usize, newy as usize)));
        } else {
            ns.push(None);
        }
    }
    ns
}

fn find_corners(
    shared_edges: &HashMap<String, HashSet<String>>,
    tile_edges: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut corners = vec![];
    for (tile_id, edges) in tile_edges {
        // println!("{} => {:?}", tile_id, edges);
        let tile_shared_edges = edges
            .iter()
            .map(|edge| shared_edges.get(edge).unwrap())
            .collect::<Vec<_>>();
        // println!("\tShared => {:?}", tile_shared_edges);

        let mut tile_set = HashSet::new();
        tile_set.insert(tile_id.clone());

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
            corners.push(tile_id.clone());
        }
    }
    corners
}

pub fn parse(
    input: &str,
) -> (
    HashMap<String, Vec<String>>,
    HashMap<String, HashSet<String>>,
    HashMap<String, Vec<String>>,
) {
    let tiles_input = input.split("\n\n");
    let mut tiles = HashMap::new();

    for tile_and_id in tiles_input {
        if tile_and_id == "" {
            continue;
        }
        let mut n = tile_and_id.lines();
        let tile_id = n.next().unwrap();
        assert!(tile_id.starts_with("Tile "));
        tiles.insert(
            tile_id[5..tile_id.len() - 1].to_owned(),
            n.map(|s| s.to_owned()).collect::<Vec<String>>(),
        );
    }

    // Any tile_id with 2 edges with only 1 shared_edge is a corner.
    let mut shared_edges = HashMap::new();
    let mut tile_edges = HashMap::new();

    for (tile_id, tile) in tiles.iter() {
        let top_edge = tile[0].to_string();
        let left_edge = tile.iter().map(|row| row.chars().next().unwrap()).join("");
        let right_edge = tile
            .iter()
            .map(|row| row.chars().last().unwrap())
            .collect::<String>();
        let bottom_edge = tile.last().unwrap().to_string();

        for edge in [top_edge, right_edge, bottom_edge, left_edge].iter() {
            let flipped_edge = edge.chars().rev().collect::<String>();

            let entry = tile_edges.entry(tile_id.to_owned()).or_insert(Vec::new());
            entry.push(edge.clone());
            entry.push(flipped_edge.clone());

            shared_edges
                .entry(edge.clone())
                .or_insert(HashSet::new())
                .insert(tile_id.to_owned());
            shared_edges
                .entry(flipped_edge.clone())
                .or_insert(HashSet::new())
                .insert(tile_id.to_owned());
        }
    }

    (tiles, shared_edges, tile_edges)
}

//
//
//
//

pub fn part1(input: &str) -> usize {
    // tiles outside have an outside border that doesn't line up with anything
    // so tiles at the corner have 2 borders that don't line up (4 when also flipped).
    // and multiply them.
    let (_, shared_edges, tile_edges) = parse(input);

    let corners = find_corners(&shared_edges, &tile_edges);
    // println!("corners => {:?}", corners);
    corners
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .product()
}

pub fn part2(input: &str) -> usize {
    // 10x10 squares, rotate 4 times, flip each time to build the
    // edges, to compose the map just dfs through the tiles looking for those with
    // shared borders rotate them until they fit and apply and go forward, if you can't find
    // an option go back and switch choice by backtracking
    let (tiles, shared_edges, tile_edges) = parse(input);

    println!("SHARED EDGES {:?}", shared_edges);

    let total_tiles = tiles.len();
    let side_len = (total_tiles as f64).sqrt().trunc() as usize;

    let mut corners = find_corners(&shared_edges, &tile_edges);

    // let sides = find_sides(&shared_edges, &tile_edges);

    let mut full_grid: Vec<Vec<Tile>> = vec![vec![vec![]; side_len]; side_len];
    let mut seen = HashSet::new();
    if !fill_grid(
        &mut full_grid,
        side_len,
        (0, 0),
        &tiles,
        &shared_edges,
        &tile_edges,
        &mut corners,
        &mut seen,
    ) {
        println!("FAILED");
    }

    printgrid(&full_grid);

    let clean_grid = full_grid
        .into_iter()
        .map(|row| {
            row.iter()
                .map(|col| remove_outer_layer(col.clone()))
                .collect()
        })
        .collect();

    let onegrid = join_all_tiles(&clean_grid, "");
    println!("{}", flipped(&rotated(&onegrid)).join("\n"));
    //     let dragon = "                  #.{len}
    // #    ##    ##    ###
    //  #  #  #  #  #  #   ";

    let line_len = onegrid[0].len();
    let dragonrex = format!(
        "..................#.{{{0}}}#....##....##....###.{{{1}}}#..#..#..#..#..#",
        line_len - 20 + 1,
        line_len - 20 + 1
    );
    println!("{}", dragonrex);
    let rex = Regex::new(&dragonrex).unwrap();
    let mut dragons = 0;
    for rotation in rotations(&onegrid) {
        dragons = rex.find_iter(&rotation.join("")).count();
        if dragons > 0 {
            println!("found {} dragons", dragons);
            dragons *= 15;
            break;
        }
    }
    onegrid.join("").chars().filter(|x| *x == '#').count() - dragons
}

fn fill_grid(
    full_grid: &mut Vec<Vec<Tile>>,
    side_len: usize,
    curr_pos: (usize, usize),
    tiles: &HashMap<String, Vec<String>>,
    shared_edges: &HashMap<String, HashSet<String>>,
    tile_edges: &HashMap<String, Vec<String>>,
    corners: &mut Vec<String>,
    seen: &mut HashSet<String>,
) -> bool {
    if curr_pos == (0, 0) {
        // get tile at pos 0 at random orientation
        // we'll iterate through all the possible orientations
        // really when we fail to find a match
        let tile_id = corners[0].clone();
        println!("tile_id {}", tile_id);
        let tile = tiles.get(&tile_id).unwrap();
        seen.insert(tile_id.clone());
        for rotation in rotations(tile) {
            full_grid[curr_pos.1][curr_pos.0] = rotation; //rotated(&rotated(&flipped(tile)));
            println!("Filling pos {:?} with tile {}:", curr_pos, tile_id);
            printgrid(full_grid);
            if fill_grid(
                full_grid,
                side_len,
                next_pos(curr_pos, side_len),
                tiles,
                shared_edges,
                tile_edges,
                corners,
                seen,
            ) {
                return true;
            }
        }
        full_grid[curr_pos.0][curr_pos.1] = Vec::new();
        // somehow no rotation was good
        return false;
    }

    println!("\n\ncurrent grid:");
    printgrid(full_grid);
    println!("Filling pos {:?}", curr_pos);

    //
    //
    //

    let neigh = neighbors(curr_pos);
    println!("neighbors of {:?}: {:?}", curr_pos, neigh);
    let default = Vec::new();
    let neighbor_tile_edges = neigh
        .iter()
        .enumerate()
        .map(|(i, &near_pos)| {
            if curr_pos == (2, 0) {
                println!("Nearby cells: {:?}", near_pos);
            }
            if near_pos.is_none() {
                return "".to_owned();
            }
            let near_pos = near_pos.unwrap();
            let neighbor = &full_grid
                .get(near_pos.1)
                .and_then(|r| r.get(near_pos.0))
                .unwrap_or(&default);

            if neighbor.is_empty() {
                return "".to_owned();
            }

            // TOP,  RIGHT,  BOTTOM,  LEFT
            let edge = match i {
                0 => bottom(&neighbor), // TOP
                1 => left(&neighbor),   // RIGHT
                2 => top(&neighbor),    // BOTTOM
                3 => right(&neighbor),  // LEFT
                _ => unreachable!(),
            };
            if curr_pos == (2, 0) {
                println!("Nearby {:?} edge: {}", near_pos, edge);
            }
            edge
        })
        .collect::<Vec<String>>();

    let mut possible_set: Vec<&HashSet<String>> = vec![];
    for (_i, neighbor_edge) in neighbor_tile_edges.iter().enumerate() {
        if neighbor_edge == "" {
            continue;
        }
        // these are candidates for this position
        possible_set.push(shared_edges.get(neighbor_edge).unwrap());
    }

    let mut probable_set: HashSet<_> = if possible_set.len() == 0 {
        return false;
    } else if possible_set.len() == 1 {
        possible_set[0].clone()
    } else {
        possible_set[1..]
            .iter()
            .fold(possible_set[0].clone(), |acc, &s| {
                &acc & s
                // acc.intersection(&s).map(|x| x.clone()).collect()
            })
    };
    probable_set = &probable_set - &seen;
    println!(
        "Here's a possible set for pos {:?} next {:?}",
        curr_pos, probable_set
    );

    for possible_tile_id in probable_set {
        let tile = tiles.get(&possible_tile_id).unwrap();
        println!("Evaluating tile_id {}", possible_tile_id);
        for rotation in rotations(tile) {
            let matches_all_edges = neighbor_tile_edges.iter().enumerate().all(|(i, edge)| {
                println!("neighbor edge at position {}: {}", i, edge);
                if edge == "" {
                    return true;
                }
                match i {
                    0 => top(&rotation) == *edge,    // TOP
                    1 => right(&rotation) == *edge,  // RIGHT
                    2 => bottom(&rotation) == *edge, // BOTTOM
                    3 => left(&rotation) == *edge,   // LEFT
                    _ => unreachable!(),
                }
            });
            if matches_all_edges {
                seen.insert(possible_tile_id.clone());
                full_grid[curr_pos.1][curr_pos.0] = rotation;
                if curr_pos == (side_len - 1, side_len - 1) {
                    // done matching
                    return true;
                }
                if fill_grid(
                    full_grid,
                    side_len,
                    next_pos(curr_pos, side_len),
                    tiles,
                    shared_edges,
                    tile_edges,
                    corners,
                    seen,
                ) {
                    return true;
                } else {
                    seen.remove(&possible_tile_id);
                    full_grid[curr_pos.1][curr_pos.0] = Vec::new();
                }
            }
        }
    }
    return false;
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
        assert_eq!(join_all_tiles(&all, ""), joined);
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
        assert_eq!(part2(&test_input), 273);
    }

    #[test]
    fn day20() {
        let input = get_input(2020, 20).unwrap();
        assert_eq!(part1(&input), 174206308298779);
        assert_eq!(part2(&input), 5);
    }
}
