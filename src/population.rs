use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::point::Point;
use rand::Rng;
use crate::individual::{draw_into_buffer, evaluate_with_buffer, initIndividual, Individual};
use crate::polygon::{CreatePolygon, Polygon};
use rand_distr::{Distribution, Normal};
pub struct Population {
    pub individuals : Vec<Individual>,
}



macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

pub fn init_population(pop_size : i16, width: u32, height: u32,start_size : i16) -> Population {
    let mut pop = vec![];
    for _i in 0..pop_size {
        pop.push(initIndividual(start_size, width, height))
    }

    let new_pop = Population{
        individuals: pop,
    };
    new_pop
}
use rayon::prelude::*;
use tiny_skia::Pixmap;
fn survive(rate : f32,  population: &mut Population, target: &Vec<u8>,width: u32, height: u32){




    // 1. Create ONE reusable buffer before the loop starts
    population.individuals.par_iter_mut().for_each_init(
        // 1. INIT: Create ONE buffer per thread
        || tiny_skia::Pixmap::new(width, height).unwrap(),

        // 2. WORK: The thread uses its specific buffer to evaluate the individual
        |buffer, ind| {
            // A. Clear the buffer
            buffer.fill(tiny_skia::Color::WHITE);

            // B. Draw into the thread's reusable buffer
            draw_into_buffer(ind, buffer);

            // C. Evaluate the fitness
            ind.fitness = evaluate_with_buffer(buffer, target);
        }
    );
    population.individuals.sort_by(|a, b| b.fitness.total_cmp(&a.fitness));

    let cut_off_index = (population.individuals.len() as f32 * rate) as usize;
    population.individuals.truncate(cut_off_index);

}


fn combine(indi1: &Individual, indi2: &Individual) -> Individual {
    let mut rng = rand::thread_rng();
    let mut child  = vec![];
    let mut out = &indi1;


    let zipped: Vec<_> = indi1.chromosomes.iter().zip(indi1.chromosomes.iter()).collect();
    let pivot = rng.gen_range(0.1..0.9);
    for shape in zipped {
        if rng.gen_range(0.0..1.0) > pivot{
            child.push(shape.0.clone());
        }
        else{
            child.push(shape.1.clone());
        }
    }

    let new_indi = Individual{
        chromosomes: child,
        fitness: out.fitness,
    };

    new_indi

}

fn select(population: &Population, size: f32) -> Vec<&Individual> {


    let mut selection = vec![];
    let mut rng = rand::thread_rng();
    let mut target_count = (population.individuals.len() as f32 * size).ceil() as usize;

    if target_count <=1 && !population.individuals.is_empty() {
        target_count = 2;
    }
    for _i in 0..target_count {
        selection.push(&population.individuals[rng.gen_range(0..population.individuals.len())]);
    }
    selection.sort_by(|a, b| b.fitness.total_cmp(&a.fitness));

    let mut parents = vec![selection[0],selection[1]];

    parents

}

use rayon::prelude::*;

pub fn run_evolution(population: &mut Population, target: &Vec<u8>, mutate_rate: f32, max_sizes: u8, width: u32, height: u32, pop_size: usize) {

    survive(0.4, population, target, width, height);


    let individuals: Vec<Individual> = (0..pop_size)
        .into_par_iter() // <--- This splits the breeding across all cores!
        .map(|_| {
            let mut rng = rand::thread_rng(); // Safe to use in threads

            let parents = select(population, 0.08);
            let mut new_indi = combine(&parents[0], &parents[1]);

            if rng.gen_range(0.0..1.0) < mutate_rate {
                mutate(&mut new_indi, 0.0, max_sizes, width, height);
            }

            new_indi
        })
        .collect();
   population.individuals = individuals;



   

}

use crate::{individual, polygon};
use rand::seq::SliceRandom;

// Notice I removed the return type -> Individual
fn mutate(individual: &mut Individual, size: f32, max_sizes: u8, width: u32, height: u32) {
    let mut rng = rand::thread_rng();

    let len = individual.chromosomes.len();

    if rng.gen_bool(0.15) && len < 1000 {
        individual.chromosomes.push(CreatePolygon(max_sizes, width as u16, height as u16));
    } else if rng.gen_bool(0.1) && len > 0 {
        individual.chromosomes.remove(rng.gen_range(0..len));
    }


    let len = individual.chromosomes.len();
    if len == 0 { return; }

    if rng.gen_bool(0.2) {
        individual.chromosomes.swap(rng.gen_range(0..len), rng.gen_range(0..len));
    } else {
        let normal = Normal::new(0.0, 20.0).unwrap();
        let poly = &mut individual.chromosomes[rng.gen_range(0..len)];

        // Mutate points
        if rng.gen_bool(0.15) {
            for p in &mut poly.points {
                p.x += normal.sample(&mut rng) as i32;
                p.y += normal.sample(&mut rng) as i32;
            }
        }

        // Mutate colors
        if rng.gen_bool(0.2) {
            let mut c = poly.color;
            c[0] = (c[0] as i32 + normal.sample(&mut rng) as i32).clamp(0, 255) as u8;
            c[1] = (c[1] as i32 + normal.sample(&mut rng) as i32).clamp(0, 255) as u8;
            c[2] = (c[2] as i32 + normal.sample(&mut rng) as i32).clamp(0, 255) as u8;
            c[3] = (c[3] as i32 + normal.sample(&mut rng) as i32).clamp(0, 255) as u8;
            poly.color = c;
        }

        // Translate all points
        if rng.gen_bool(0.1) {
            let x_change = normal.sample(&mut rng) as i32;
            let y_change = normal.sample(&mut rng) as i32;
            for p in &mut poly.points {
                p.x += x_change;
                p.y += y_change;
            }
        }
    }
}









