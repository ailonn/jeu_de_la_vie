const MAX_ITERATION: u32 = 10;

const MAX_X:usize = 25;
const MAX_Y:usize = 100;

const MID_X_INDEX : usize = (MAX_X - 1)/2;
const MID_Y_INDEX : usize = (MAX_Y - 1)/2;

const FULL : char = 'O';
const EMPTY : char = ' ';

type Map = [[char; MAX_Y]; MAX_X];

// gérer la vie des cellules via le `lifetime ?
// utiliser les traits pour spécialiser les cellules ?

/* une case est vide => solicitation du gestionnaire de vie
    en fonction du voisinage il va peupler la case
    attribution d'une probabilité d'apparition d'une entité d'un type donné
    puis tirage au hasard
*/
/* la case n'est pas vide =>
    en fonction du voisinage perçus  l'entité évolue
*/

// une entité : structure (Point ?) + Trait en fonction du type d'entité
/* gestionnaire de vie : module avec une structure définissant les types d'entités, leur condition d'apparition et un constructeur
 leur probabilité d'apparition est géré par un Trait ? 
*/
// pour la doc : `cargo doc`


fn main() {
    jeu_de_la_vie();
}

fn print_the_map(world : &Map) {
    for x_line in world.iter() {
        print!("|");
        for y_case in x_line.iter() {
            print!("{}", y_case);
        }
        println!("|");
    }
}

fn if_edge_else(index: usize, limit: usize) -> usize {
    if index == 0 {limit - 1} else {index - 1}
}

fn if_limit_else(index: usize, limit: usize) -> usize {
    if index == limit - 1 {0} else {index + 1}
}

//fn slice_map(world, case.perception, idx_x, idx_y) {

//}

fn update_the_map(world : &Map) -> Map {
    let mut updated_map : Map = world.clone();

    let mut idx_x = 0;
    for line in world.iter() {
        let mut idx_y = 0;
        //print!("|");
        for case in line.iter() {
  //          case.you_see(slice_map(world, case.perception, idx_x, idx_y));
            let mut neighborhood = 0;
            if world[if_edge_else(idx_x, MAX_X)][if_edge_else(idx_y, MAX_Y)] == FULL {
                neighborhood += 1;
            }
            if world[idx_x][if_edge_else(idx_y, MAX_Y)] == FULL {
                neighborhood += 1;
            }
            if world[if_edge_else(idx_x, MAX_X)][idx_y] == FULL {
                neighborhood += 1;
            }
            if world[if_edge_else(idx_x, MAX_X)][if_limit_else(idx_y, MAX_Y)] == FULL {
                neighborhood += 1;
            }
            if world[if_limit_else(idx_x, MAX_X)][if_edge_else(idx_y, MAX_Y)] == FULL {
                neighborhood += 1;
            }
            if world[if_limit_else(idx_x, MAX_X)][if_limit_else(idx_y, MAX_Y)] == FULL {
                neighborhood += 1;
            }
            if world[idx_x][if_limit_else(idx_y, MAX_Y)] == FULL {
                neighborhood += 1;
            }
            if world[if_limit_else(idx_x, MAX_X)][idx_y] == FULL {
                neighborhood += 1;
            }
            if neighborhood < 2 {
                updated_map[idx_x][idx_y] = EMPTY;
            } else if neighborhood > 3 {
                updated_map[idx_x][idx_y] = EMPTY;
            } else /*if neighborhood == 3*/  {
                updated_map[idx_x][idx_y] = FULL;
            }
            //print!("{}", neighborhood);
            idx_y += 1;
        }
        //println!("|");
        idx_x +=1
    }
    updated_map
}

fn jeu_de_la_vie() {
    let mut world: Map  = [[EMPTY;MAX_Y];MAX_X];
    println!("Hello, world!");

    let mut iteration_count: u32 = 0;

    world[MID_X_INDEX][MID_Y_INDEX + 1] = FULL;
    world[MID_X_INDEX+1][MID_Y_INDEX + 1] = FULL;
    world[MID_X_INDEX+1][MID_Y_INDEX] = FULL;
    world[MID_X_INDEX+1][MID_Y_INDEX - 1] = FULL;
    world[MID_X_INDEX-1][MID_Y_INDEX - 1] = FULL;

    while iteration_count < MAX_ITERATION {
        iteration_count += 1;
        print_the_map(&world);
        world = update_the_map(&world);
        println!("iteration numméro : {}", iteration_count);
    }
    println!("Bye world!");
}