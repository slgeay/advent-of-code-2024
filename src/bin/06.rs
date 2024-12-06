advent_of_code::solution!(6);

pub fn part_one(i: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = i.lines().map(|s| s.chars().collect()).collect();
    let (h, w) = (grid.len() as i32, grid[0].len() as i32);
    let vec = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().enumerate().find_map(|(j, &c)| (c == '^').then_some((i, j))))?;

    let mut count = 1;

    loop {
        let (nx, ny) = (x as i32 + vec[dir].0, y as i32 + vec[dir].1);
        if nx < 0 || nx >= h || ny < 0 || ny >= w {
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        match grid[nx][ny] {
            '#' => {
                dir = (dir + 1) % 4;
                continue;
            }
            '.' => {
                count += 1;
                grid[nx][ny] = 'X'
            }
            _ => {}
        }
        (x, y) = (nx, ny);
    }
    Some(count)
}

pub fn part_two(i: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = i.lines().map(|s| s.chars().collect()).collect();
    let (h, w) = (grid.len() as i32, grid[0].len() as i32);
    let vec = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let flag = [1u8, 2, 4, 8];
    let mut dir = 0;
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().enumerate().find_map(|(j, &c)| (c == '^').then_some((i, j))))?;

    let mut count = 0;
    grid[x][y] = flag[dir] as char;

    loop {
        let (nx, ny) = (x as i32 + vec[dir].0, y as i32 + vec[dir].1);
        if nx < 0 || nx >= h || ny < 0 || ny >= w {
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        match grid[nx][ny] {
            '#' => {
                dir = (dir + 1) % 4;
                continue;
            }
            '.' => {
                let mut vgrid = grid.clone();
                let mut vdir = (dir + 1) % 4;
                vgrid[nx][ny] = '#';
                let (mut vx, mut vy) = (x, y);
                loop {
                    let (vnx, vny) = (vx as i32 + vec[vdir].0, vy as i32 + vec[vdir].1);
                    if vnx < 0 || vnx >= h || vny < 0 || vny >= w {
                        break;
                    }
                    let (vnx, vny) = (vnx as usize, vny as usize);
                    match vgrid[vnx][vny] {
                        '#' => {
                            vdir = (vdir + 1) % 4;
                            continue;
                        }
                        '.' => {
                            vgrid[vnx][vny] = flag[vdir] as char;
                        }
                        c => {
                            if (c as u8 & flag[vdir]) != 0 {
                                count += 1;
                                break;
                            }
                            vgrid[vnx][vny] = (c as u8 | flag[vdir]) as char;
                        }
                    }
                    (vx, vy) = (vnx, vny);
                }

                grid[nx][ny] = flag[dir] as char;
            }
            c => {
                grid[nx][ny] = (c as u8 | flag[dir]) as char;
            }
        }
        (x, y) = (nx, ny);
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
