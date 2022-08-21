mod data_structures;

use crate::data_structures::{QuadTree, QuadTreeBoundary};
use rand::{thread_rng, Rng};
use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex};
use sfml::window::{Event, Key, Style};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 400;

const N_VERTICES: usize = 2_usize.pow(8);
const QUERY_BOX_HALF_LENGTH: f32 = 20.;

fn main() {
    let mut vertices = [Vertex::default(); N_VERTICES];
    vertices.iter_mut().for_each(|v| {
        v.position.x = thread_rng().gen_range(0..WIDTH) as f32;
        v.position.y = thread_rng().gen_range(0..HEIGHT) as f32;
    });

    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Quadtree",
        Style::default(),
        &Default::default(),
    );

    window.set_framerate_limit(60);
    while window.is_open() {
        // event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::KeyPressed { code: Key::P, .. } => {
                    while let Some(event) = window.wait_event() {
                        if let Event::KeyReleased { code: Key::P, .. } = event {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        // tick
        vertices.iter_mut().for_each(|v| {
            v.position.x += thread_rng().gen_range(-1..=1) as f32;
            v.position.y += thread_rng().gen_range(-1..=1) as f32;

            v.position.x = v.position.x.abs() % WIDTH as f32;
            v.position.y = v.position.y.abs() % HEIGHT as f32;
        });

        let mut quad_tree = QuadTree::new(WIDTH as f32, HEIGHT as f32);
        vertices.iter().for_each(|v| quad_tree.insert(v));

        let mouse_position = window.mouse_position();
        let (mouse_x, mouse_y) = (mouse_position.x as f32, mouse_position.y as f32);
        let query_box = QuadTreeBoundary::new(
            mouse_x - QUERY_BOX_HALF_LENGTH,
            mouse_y - QUERY_BOX_HALF_LENGTH,
            mouse_x + QUERY_BOX_HALF_LENGTH,
            mouse_y + QUERY_BOX_HALF_LENGTH,
        );
        let vertices_in_query_box = quad_tree.query(&query_box);

        let jura = vertices_in_query_box.iter().map(|v| Vertex::with_pos_color(v.position, Color::RED)).collect::<Vec<Vertex>>();
        let jura = jura.as_slice();

        // rendering
        window.clear(Color::BLACK);
        window.draw_primitives(&vertices, PrimitiveType::POINTS, &RenderStates::DEFAULT);

        window.draw_primitives(jura, PrimitiveType::POINTS, &RenderStates::DEFAULT);
        window.draw(&query_box);

        window.draw(&quad_tree);
        window.display();
    }
}

