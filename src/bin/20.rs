use std::mem;
use smallvec::{smallvec, SmallVec};
use itertools::Itertools;

const INPUT: &'static str = include_str!("inputs/20.txt");

type Tile<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Stmt {
    tile_side: usize,
    tiles: Vec<(usize, Tile<bool>)>
}

fn stmt(input: &str) -> Option<Stmt> {
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let tiles = lines.split(|line| line.is_empty());

    let tiles = tiles.map(|block| {
        let id = block.get(0)?.strip_prefix("Tile ")?.strip_suffix(":")?.parse::<usize>().ok()?;
        let tile = block.get(1..)?.iter().map(|line| line.chars().map(|c| c == '#').collect()).collect();

        Some((id, tile))
    }).collect::<Option<Vec<(usize, Tile<bool>)>>>()?;

    let tile_side = tiles.get(0).map_or(0, |(_, f)| f.len());

    if tiles.iter().any(|(_, t)| t.len() != tile_side || t.iter().any(|l| l.len() != tile_side)) {
        return None;
    }

    Some(Stmt { tile_side, tiles })
}

#[derive(Clone, Copy, Debug)]
struct Borders {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32
}

impl Borders {
    fn to_smallvec(&self) -> SmallVec<[u32; 4]> {
        smallvec![self.left, self.right, self.top, self.bottom]
    }
}

type Variants = [Borders; 8];

fn b_to_usize(bools: impl Iterator<Item=bool>) -> u32 {
    bools.fold(0u32, |a, c| a + a + (c as u32))
}

fn b_reverse(tile_side: usize, code: u32) -> u32 {
    u32::reverse_bits(code) >> (32 - tile_side as u32)
}

fn borders(tile_side: usize, t: &Tile<bool>) -> Borders {
    let top = b_to_usize((0..tile_side).map(|i| t[0][i]));
    let bottom = b_to_usize((0..tile_side).map(|i| t[tile_side-1][i]));
    let left = b_to_usize((0..tile_side).map(|i| t[i][0]));
    let right = b_to_usize((0..tile_side).map(|i| t[i][tile_side-1]));

    Borders { left, right, top, bottom }
}

fn variants(tile_side: usize, t: &Tile<bool>) -> Variants {
    let mut ret = [borders(tile_side, t); 8];
    for ti in 1..8 {
        let t = &mut ret[ti];

        if ti & 1 != 0 {
            mem::swap(&mut t.left, &mut t.right);
            t.top = b_reverse(tile_side, t.top);
            t.bottom = b_reverse(tile_side, t.bottom);
        }

        if ti & 2 != 0 {
            mem::swap(&mut t.top, &mut t.bottom);
            t.left = b_reverse(tile_side, t.left);
            t.right = b_reverse(tile_side, t.right);
        }

        if ti & 4 != 0 {
            mem::swap(&mut t.top, &mut t.left);
            mem::swap(&mut t.bottom, &mut t.right);
        }
    }

    ret
}

fn tile_flip<A: Copy>(t: &mut Tile<A>, ti: usize) {
    assert!(ti < 8);

    if ti & 1 != 0 {
        for row in t.iter_mut() {
            row.reverse();
        }
    }

    if ti & 2 != 0 {
        t.reverse();
    }

    if ti & 4 != 0 {
        let l = t.len();
        for i in 0..l {
            for j in (i+1)..l {
                let tmp = t[i][j];
                t[i][j] = t[j][i];
                t[j][i] = tmp;
            }
        }
    }
}

fn tile_unflip<A: Copy>(t: &mut Tile<A>, ti: usize) {
    assert!(ti < 8);

    if ti < 4 {
        tile_flip(t, ti);

    } else {
        // yeah i know but tired to prove it at 1am
        tile_flip(t, 4);
        tile_flip(t, ti & 3);

    }
}

fn tile_crop(t: &mut Tile<bool>) {
    t.pop();
    t.remove(0);
    for row in t.iter_mut() {
        row.pop();
        row.remove(0);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Waters {
    Spaces,
    Hash,
    Monster
}

fn stitch_tiles(image_side: usize, tile_side: usize, tiles: &[Tile<bool>]) -> Tile<Waters> {
    tiles.chunks(image_side).flat_map(|tile_row| {
        (0..tile_side).map(move |row| {
            tile_row.iter().flat_map(|tile| tile[row].iter().map(|&f| if f { Waters::Hash } else { Waters::Spaces })).collect()
        })
    }).collect()
}

type Assignment = Vec<(usize, usize)>;

fn pruning_search(variants: &[(usize, Variants)]) -> Option<(usize, Assignment)> {
    let image_side = (0..).take_while(|s| s*s <= variants.len()).find(|s| s*s == variants.len())?;

    fn rec(
        variants: &[(usize, Variants)],
        assignment: &mut Assignment,
        used: &mut [bool],
        bottoms: &mut Vec<u32>,
        right: u32,
        image_side: usize) -> Result<(), Assignment>
    {
        let k = assignment.len();

        if k == image_side * image_side {
            return Err(assignment.clone());
        }

        let (i, j) = (k / image_side, k % image_side);

        for (vi, &(id, variant)) in variants.iter().enumerate() {
            if !used[vi] {
                used[vi] = true;

                for (ti, borders) in variant.iter().enumerate() {
                    if i == 0 || borders.top == bottoms[k - image_side] {
                        if j == 0 || borders.left == right {
                            assignment.push((id, ti));
                            bottoms.push(borders.bottom);

                            rec(variants, assignment, used, bottoms, borders.right, image_side)?;

                            bottoms.pop();
                            assignment.pop();
                        }
                    }
                }

                used[vi] = false;
            }
        }

        Ok(())
    }

    let mut assignment = vec![];
    let mut used = vec![false; variants.len()];
    let mut bottoms = vec![];

    let ids = rec(variants, &mut assignment, &mut used, &mut bottoms, 0, image_side).err()?;

    Some((image_side, ids))
}

const SEA_MONSTER: [[bool; 20]; 3] = [
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false],
    [true, false, false, false, false, true, true, false, false, false, false, true, true, false, false, false, false, true, true, true],
    [false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false]
];

fn mark_seamonsters(image: &mut Tile<Waters>) {
    let iw = image.len();

    for ti in 0..8 {
        tile_flip(image, ti);

        for i in 0..(iw-3) {
            for j in 0..(iw-20) {
                let mut found = true;

                'outer: for di in 0..3 {
                    for dj in 0..20 {
                        if SEA_MONSTER[di][dj] && image[i+di][j+dj] == Waters::Spaces {
                            found = false;
                            break 'outer;
                        }
                    }
                }

                if !found {
                    continue;
                }

                for di in 0..3 {
                    for dj in 0..20 {
                        if SEA_MONSTER[di][dj] {
                            image[i+di][j+dj] = Waters::Monster;
                        }
                    }
                }
            }
        }

        tile_unflip(image, ti);
    }
}

fn part_two(input: &str) -> usize {
    let stmt = stmt(input).expect("correct parse");
    let variants: Vec<_> = stmt.tiles.iter().map(|(id, t)| (*id, variants(stmt.tile_side, t))).collect();
    let (image_side, ids) = pruning_search(&variants[..]).expect("solution exists");

    let tiles = ids.into_iter().map(|(id, ti)| {
        stmt.tiles.iter().find_map(|(tile_id, tile)| {
            if id != *tile_id {
                return None;
            }

            let mut tile = tile.clone();
            tile_flip(&mut tile, ti);
            tile_crop(&mut tile);

            Some(tile)
        })
    }).collect::<Option<Vec<_>>>().expect("search works, qed.");

    let mut image = stitch_tiles(image_side, stmt.tile_side - 2, &tiles);

    mark_seamonsters(&mut image);

    image.into_iter().flat_map(|row| row).filter(|&p| p == Waters::Hash).count()
}

fn part_one(input: &str) -> usize {
    let stmt = stmt(input).expect("correct parse");

    let borders: Vec<Borders> = stmt.tiles.iter()
        .map(|(_, t)| borders(stmt.tile_side, t))
        .collect();

    let edges: Vec<u32> = borders.iter()
        .flat_map(|b| b.to_smallvec())
        .map(|bin| bin.min(b_reverse(stmt.tile_side, bin)))
        .collect();

    let unique_edges: Vec<u32> = edges.into_iter()
        .sorted()
        .group_by(|&k| k)
        .into_iter()
        .filter_map(|(edge, group)| {
            if group.count() > 1 { None } else { Some(edge) }
        })
        .collect();

    let tile_ids = unique_edges.into_iter()
        .map(|edge| {
            let pos = borders.iter().position(|b| {
                let sv = b.to_smallvec();
                sv.contains(&edge) || sv.contains(&b_reverse(stmt.tile_side, edge))
            })?;

            let &(id, _) = stmt.tiles.get(pos)?;
            Some(id)
        })
        .collect::<Option<Vec<usize>>>().expect("all belong to borders");

    tile_ids.into_iter()
        .sorted()
        .group_by(|&k| k)
        .into_iter()
        .filter_map(|(edge, group)| {
            if group.count() == 2 { Some(edge) } else { None }
        })
        .product()
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example() {
    let input = r"Tile 2311:
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
..#.###...";

    assert_eq!(part_one(&input), 20899048083289);
    assert_eq!(part_two(&input), 273);
}
