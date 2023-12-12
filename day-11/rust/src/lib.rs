use itertools::Itertools;


fn get_exp_galaxy_pos(input: &str, exp_factor: usize) -> Vec<(usize,usize)> {
    let mut exp_col = Vec::new();
    let mut exp_row = Vec::new();
    let mut galaxies = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        exp_row.push(true);
        line.chars().enumerate().for_each(|(x, ch)| {
            exp_col.push(true);
            if ch == '#' {
                exp_row[y] = false;
                exp_col[x] = false;
                galaxies.push((y,x))
            }
        });
    });

    galaxies.iter().map(|(y,x)| {
        let y_add =  (exp_factor -1) * exp_row.iter().enumerate().filter(|(idx, should_expand)| idx < y && **should_expand).collect::<Vec<_>>().len() ;
        let x_add =  (exp_factor -1) * exp_col.iter().enumerate().filter(|(idx, should_expand)| idx < x && **should_expand).collect::<Vec<_>>().len() ;

        (y+y_add,x+x_add)
    }).collect()

}

pub fn process_part1(input: &str) -> String {
    let galaxies = get_exp_galaxy_pos(input, 2);
    let res = galaxies.into_iter().combinations(2).map(|v| {
        let a = v[0];
        let b = v[1];
        //calc distance 
        let x_dist = std::cmp::max(a.1, b.1) - std::cmp::min(a.1, b.1);
        let y_dist = std::cmp::max(a.0, b.0) - std::cmp::min(a.0, b.0);
        x_dist + y_dist
    }).sum::<usize>();

    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    let galaxies = get_exp_galaxy_pos(input, 1000000);
    let res = galaxies.into_iter().combinations(2).map(|v| {
        let a = v[0];
        let b = v[1];
        //calc distance 
        let x_dist = std::cmp::max(a.1, b.1) - std::cmp::min(a.1, b.1);
        let y_dist = std::cmp::max(a.0, b.0) - std::cmp::min(a.0, b.0);
        x_dist + y_dist
    }).sum::<usize>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "374");
    }

    #[test]
    //#[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        // This would be the test for factor 100, but the actual factor is 1000000 
        // assert_eq!(result, "8410");
        assert_eq!(result,"82000210");
    }
}
