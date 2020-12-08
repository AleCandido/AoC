use aocinput::request;

struct Trajectory(usize, usize);

fn main() {
    let resp = request::get_input(2020, 3);
    let trees: Vec<Vec<char>> = resp
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    let width = trees[0].len();

    let trajectories: Vec<Trajectory> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|t| Trajectory(t.0, t.1))
        .collect();

    let mut trees_crossed: Vec<usize> = vec![];

    for traj in trajectories.iter() {
        let mut x = 0;
        let mut trees_current = 0;

        for row in trees.iter().step_by(traj.1) {
            if row[x] == '#' {
                trees_current += 1;
            }
            x = (x + traj.0) % width;
        }

        trees_crossed.push(trees_current);
    }

    println!("#trees crossed: {:#?}", trees_crossed);
    println!("product: {:#?}", trees_crossed.iter().product::<usize>());
}
