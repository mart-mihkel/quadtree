mod data_structures;

use crate::data_structures::QuadTree;
use rand::{thread_rng, Rng};
use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex};
use sfml::system::Vector2f;
use sfml::window::{Event, Key, Style};

const WIDTH: f32 = 1000.;
const HEIGHT: f32 = 600.;

const N_VERTICES: usize = 2_usize.pow(12);
const LOOKUP_RADIUS: f32 = 25.;

fn main() {
    // initial positions and velocities
    let mut velocities = [Vector2f::default(); N_VERTICES];
    let mut vertices = [Vertex::default(); N_VERTICES];
    vertices.iter_mut().zip(velocities.iter_mut()).for_each(|(ver, vel)| {
        ver.position.x = thread_rng().gen_range((3. * WIDTH / 4.)..WIDTH);
        ver.position.y = thread_rng().gen_range((3. * HEIGHT / 4.)..HEIGHT);

        vel.x = thread_rng().gen_range(-1.0..=1.0);
        vel.y = thread_rng().gen_range(-1.0..=1.0);
    });

    let mut window = RenderWindow::new(
        (WIDTH as u32, HEIGHT as u32),
        "quadtree",
        Style::DEFAULT,
        &Default::default(),
    );

    // rendering options
    let mut paused = false;
    let mut draw_tree = true;
    let mut draw_vertices = true;
    let mut highlight_searched = false;

    window.set_framerate_limit(60);
    while window.is_open() {
        // event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::KeyPressed { code: Key::P, .. } => paused = !paused,
                Event::KeyPressed { code: Key::T, .. } => draw_tree = !draw_tree,
                Event::KeyPressed { code: Key::V, .. } => draw_vertices = !draw_vertices,
                Event::KeyPressed { code: Key::H, .. } => highlight_searched = !highlight_searched,
                _ => {}
            }
        }

        // tick
        if !paused {
            vertices.iter_mut().zip(velocities.iter_mut()).for_each(|(ver, vel)| {
                let angle: f32 = thread_rng().gen_range(-0.25..0.25);
                let (angle_sin, angle_cos) = angle.sin_cos();

                let (vel_x, vel_y) = (vel.x, vel.y);
                vel.x = vel_x * angle_cos - vel_y * angle_sin;
                vel.y = vel_x * angle_sin + vel_y * angle_cos;

                ver.position += *vel;

                ver.position.x = (ver.position.x + WIDTH) % WIDTH;
                ver.position.y = (ver.position.y + HEIGHT) % HEIGHT;
            });
        }

        let mut quad_tree = QuadTree::new(WIDTH, HEIGHT, highlight_searched);
        vertices.iter().for_each(|v| quad_tree.insert(v));

        let mouse_position = window.mouse_position();
        let (mouse_x, mouse_y) = (mouse_position.x as f32, mouse_position.y as f32);

        let mut vertices_in_radius = Vec::new();
        quad_tree.lookup(mouse_x, mouse_y, LOOKUP_RADIUS, &mut vertices_in_radius);

        let vertices_in_radius = vertices_in_radius
            .iter()
            .map(|v| Vertex::with_pos_color(Vector2f::new(v.position.x, v.position.y), Color::rgb(255, 60, 0)))
            .collect::<Vec<Vertex>>();

        // rendering
        window.clear(Color::BLACK);

        if draw_vertices {
            window.draw_primitives(&vertices, PrimitiveType::POINTS, &RenderStates::DEFAULT);
            window.draw_primitives(vertices_in_radius.as_slice(), PrimitiveType::POINTS, &RenderStates::DEFAULT);
        }

        if draw_tree { window.draw(&quad_tree); }

        window.display();
    }
}

