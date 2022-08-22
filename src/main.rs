mod data_structures;

use crate::data_structures::{QuadTree, QuadTreeBoundary};
use rand::{thread_rng, Rng};
use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Vertex};
use sfml::system::Vector2f;
use sfml::window::{Event, Key, Style};

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 400.;

const N_VERTICES: usize = 2_usize.pow(12);
const QUERY_BOX_HALF_LENGTH: f32 = 20.;

fn main() {
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
        "QuadTree",
        Style::DEFAULT,
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
                        if let Event::KeyPressed { code: Key::P, .. } = event {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        // tick
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

        let mut quad_tree = QuadTree::new(WIDTH, HEIGHT);
        vertices.iter().for_each(|v| quad_tree.insert(v));

        let mouse_position = window.mouse_position();
        let (mouse_x, mouse_y) = (mouse_position.x as f32, mouse_position.y as f32);
        let query_box = QuadTreeBoundary::new(
            mouse_x - QUERY_BOX_HALF_LENGTH,
            mouse_y - QUERY_BOX_HALF_LENGTH,
            mouse_x + QUERY_BOX_HALF_LENGTH,
            mouse_y + QUERY_BOX_HALF_LENGTH,
        );

        let vertices_in_query_box = quad_tree
            .query(&query_box)
            .iter()
            .map(|v| Vertex::with_pos_color(v.position, Color::RED))
            .collect::<Vec<Vertex>>();
        let vertices_in_query_box = vertices_in_query_box.as_slice();

        // rendering
        window.clear(Color::BLACK);
        window.draw_primitives(&vertices, PrimitiveType::POINTS, &RenderStates::DEFAULT);
        window.draw_primitives(vertices_in_query_box, PrimitiveType::POINTS, &RenderStates::DEFAULT);
        window.draw_primitives(
            &[
                Vertex::with_pos_color(Vector2f::new(mouse_x - QUERY_BOX_HALF_LENGTH, mouse_y - QUERY_BOX_HALF_LENGTH), Color::RED),
                Vertex::with_pos_color(Vector2f::new(mouse_x + QUERY_BOX_HALF_LENGTH, mouse_y - QUERY_BOX_HALF_LENGTH), Color::RED),
                Vertex::with_pos_color(Vector2f::new(mouse_x + QUERY_BOX_HALF_LENGTH, mouse_y + QUERY_BOX_HALF_LENGTH), Color::RED),
                Vertex::with_pos_color(Vector2f::new(mouse_x - QUERY_BOX_HALF_LENGTH, mouse_y + QUERY_BOX_HALF_LENGTH), Color::RED),
                Vertex::with_pos_color(Vector2f::new(mouse_x - QUERY_BOX_HALF_LENGTH, mouse_y - QUERY_BOX_HALF_LENGTH), Color::RED),
            ],
            PrimitiveType::LINE_STRIP,
            &RenderStates::DEFAULT,
        );
        window.draw(&quad_tree);
        window.display();
    }
}

