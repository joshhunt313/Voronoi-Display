use voronoice::*;
use rand::Rng;
use plotters::prelude::*;
use plotters::element::Polygon;
use std::env;

/**
 * Generates a random point inside the given bounding box
 */
fn gen_rand_point(bounding_box: BoundingBox) -> Point {
    let mut rng = rand::thread_rng();
    Point { x: rng.gen_range(bounding_box.left()..bounding_box.right()), y: rng.gen_range(bounding_box.top()..bounding_box.bottom())}
}

/**
 * Gets the number of sites if specified in the command line arguments
 */
fn get_num_sites() -> Option<i32> {
    let args: Vec<String> = env::args().collect();
    let mut iter = args.into_iter();
    while let Some(string) = iter.next() {
        if string == "-s" {
            let num_opt = match iter.next() {
                Some(num) => Some(num),
                None => None
            };
            
            if num_opt != None {
                let num = num_opt.unwrap().parse().unwrap();
                return Some(num);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate specified number of random points and store them into the sites vector
    let num_sites = match get_num_sites() {
                            Some(num) => num,
                            None => 10
                        };
    let mut sites: Vec<Point> = vec![];
    for _ in 0..num_sites{
        sites.push(gen_rand_point(BoundingBox::new_centered_square(10.)));
    }

    // Build a voronoi diagram from the set of random points
    let my_voronoi = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(BoundingBox::new_centered_square(10.))
        .set_lloyd_relaxation_iterations(5)
        .build()
        .unwrap();

    // Initialize drawing area
    let root = SVGBackend::new("0.svg", (500, 500)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // For each cell in the voronoi diagram, fit it to the final image, and then color in the corresponding space
    for cell in my_voronoi.iter_cells() {
        // Map verticies to final image
        let vertices: Vec<(i32, i32)> = cell.iter_vertices()
            .map(|p| ((p.x * 50. + 250.) as i32, (p.y * 50. + 250.) as i32))
            .collect();

        // Make cell color random
        let mut rng = rand::thread_rng();
        let cell_color = RGBAColor(rng.gen(), rng.gen(), rng.gen(), 1.);

        // Set the style for each cell
        let style = ShapeStyle {
            // Change the color depending on the cell number
            color: cell_color,
            filled: true,
            stroke_width: 2,
        };

        // Draw the cell on the final image and increment the color index
        root.draw(&Polygon::new(vertices, style))?;
    }

    root.present()?;

    Ok(())


}
