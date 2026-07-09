mod polygon;
mod population;
mod individual;


use image::{GenericImageView, Rgba, RgbaImage};
use js_sys::Math::{fround, round};
// Switched to Rgba
use std::time::Instant;
use crate::individual::{draw_into_buffer, initIndividual, Individual};
use crate::population::{init_population, run_evolution, Population,uniform_crossover,array_based_pivot};




macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = args.get(1).map(|s| s.as_str()).unwrap_or("9c.png");
    let generations = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let pop_size = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(350);

    let img_path = format!("images/{}", file_name);
    let target_img = image::open(&img_path).expect("Could not find image");
    let (width, height) = target_img.dimensions();

    let target_bytes = target_img.to_rgba8().into_raw();

    println!("init ({}x{})...", width, height);
    let start = Instant::now();

    let mut engine = EvolutionEngine::new(&target_bytes, width, height, generations, pop_size);
    let best_pixels = engine.run_generations();

    println!("Saving output");
    let output_img = image::RgbaImage::from_raw(width, height, best_pixels)
        .expect("Failed to create output image");

    let best_ind = &engine.population.individuals[0];
    let size = best_ind.chromosomes.len();
    let rounded_fitness = (best_ind.fitness * 1000.0).round() / 1000.0;

    let path = format!("out/{file_name}_{generations}_{pop_size}_{size}_{rounded_fitness}.png");
    println!("{}",path);
    output_img.save(path).unwrap();

    println!("Done. Time elapsed: {:?}", start.elapsed());


    //RUSTFLAGS="-C target-cpu=native" cargo run --release -- 9c.png 10000 500
}


pub struct EvolutionEngine {
    population: Population,
    target_bytes: Vec<u8>,
    generations: i32,
    pop_size: i32,
    width: u32,
    height: u32
}


impl EvolutionEngine {


    pub fn new(image_data: &[u8], width: u32, height: u32,generations : i32, pop_size :i32) -> EvolutionEngine {
        console_error_panic_hook::set_once();
        let mut population = init_population(pop_size as i16, width, height, 30);
        EvolutionEngine{
            population,
            target_bytes: image_data.to_vec(),
            width,
            height,
            generations,
            pop_size
        }


    }

    pub fn run_generations(&mut self)->Vec<u8> {
        let mut mutation_mul = 1.5;
        let mut mutation_rate = 0.9;
        let mut shape_size_mul = 1.4;
        let mut  survival_rate = 0.3;

        let mut combine_function: fn(&Individual, &Individual) -> Individual = uniform_crossover;

        combine_function = array_based_pivot;
        for i in 0..self.generations {

            let best_fitness = self.population.individuals[0].fitness;
            if best_fitness > 0.952 {
                mutation_mul = 0.5;
                mutation_rate = 0.5;
                shape_size_mul = 0.5;
                survival_rate = 0.4;
                combine_function = array_based_pivot;
            }

            else if best_fitness > 0.94 {
                mutation_mul = 1.0;
                mutation_rate = 0.6;
                shape_size_mul = 0.8;
                survival_rate = 0.35;
                combine_function = array_based_pivot;
            }


            else if best_fitness > 0.92 {
                mutation_mul = 1.35;
                mutation_rate = 0.8;
                shape_size_mul = 1.0;
            }



            if i %50 == 0 {
                let best_length = self.population.individuals[0].chromosomes.len();
                println!("{} Generation {} best fitness {} ", best_length,i , best_fitness);
            }
            run_evolution(
                &mut self.population,
                &self.target_bytes,
                mutation_rate,
                7,
                self.width,
                self.height,
                self.pop_size as usize,
                mutation_mul,
                shape_size_mul,
                survival_rate,
                combine_function

            );
        }
        self.population.individuals.sort_by(|a, b| b.fitness.total_cmp(&a.fitness));
        let best_individual = &self.population.individuals[0];


        let mut best_buffer = tiny_skia::Pixmap::new(self.width, self.height).unwrap();
        best_buffer.fill(tiny_skia::Color::WHITE);
        draw_into_buffer(best_individual, &mut best_buffer);
        best_buffer.data().to_vec()
    }
}


