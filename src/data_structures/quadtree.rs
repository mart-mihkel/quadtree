use sfml::graphics::{Color, Drawable, PrimitiveType, RenderStates, RenderTarget, Vertex};
use sfml::system::Vector2f;

const CAPACITY: usize = 5;
const SEARCHED_COLOR: Color = Color::rgba(255, 60, 0, 64);

pub struct QuadTreeBoundary {
    left_x: f32,
    top_y: f32,
    right_x: f32,
    bottom_y: f32,
    been_searched: bool,
}

impl QuadTreeBoundary {
    pub fn new(left_x: f32, top_y: f32, right_x: f32, bottom_y: f32) -> Self {
        let been_searched = false;

        Self { left_x, top_y, right_x, bottom_y, been_searched }
    }

    pub fn contains(&self, point: &Vertex) -> bool {
        point.position.x >= self.left_x && point.position.x <= self.right_x && point.position.y >= self.top_y && point.position.y <= self.bottom_y
    }

    pub fn overlaps(&self, x: f32, y: f32, r: f32) -> bool {
        let x_distance = x - x.max(self.left_x).min(self.right_x);
        let y_distance = y - y.max(self.top_y).min(self.bottom_y);

        return x_distance.powf(2.) + y_distance.powf(2.) <= r.powf(2.);
    }
}

impl Drawable for QuadTreeBoundary {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let centre_x = (self.right_x + self.left_x) / 2.;
        let centre_y = (self.bottom_y + self.top_y) / 2.;

        if self.been_searched {
            target.draw_primitives(
                &[
                    Vertex::with_pos_color(Vector2f::new(self.left_x, self.top_y), SEARCHED_COLOR),
                    Vertex::with_pos_color(Vector2f::new(self.right_x, self.top_y), SEARCHED_COLOR),
                    Vertex::with_pos_color(Vector2f::new(self.right_x, self.bottom_y), SEARCHED_COLOR),
                    Vertex::with_pos_color(Vector2f::new(self.left_x, self.bottom_y), SEARCHED_COLOR),
                ],
                PrimitiveType::QUADS,
                states,
            );
        }

        target.draw_primitives(
            &[
                Vertex::with_pos(Vector2f::new(self.left_x, centre_y)),
                Vertex::with_pos(Vector2f::new(self.right_x, centre_y)),
                Vertex::with_pos(Vector2f::new(centre_x, self.top_y)),
                Vertex::with_pos(Vector2f::new(centre_x, self.bottom_y)),
            ],
            PrimitiveType::LINES,
            states,
        );
    }
}

pub struct QuadTree<'q> {
    items: Vec<&'q Vertex>,
    children: Vec<QuadTree<'q>>,
    boundary: QuadTreeBoundary,
    draw_searched: bool,
}

impl<'q> QuadTree<'q> {
    pub fn new(width: f32, height: f32, draw_searched: bool) -> Self {
        Self::new_with_boundary(QuadTreeBoundary::new(0., 0., width, height), draw_searched)
    }

    fn new_with_boundary(boundary: QuadTreeBoundary, draw_searched: bool) -> Self {
        let items = Vec::with_capacity(CAPACITY);
        let children = Vec::with_capacity(4);

        Self { items, children, boundary, draw_searched }
    }

    pub fn insert(&mut self, item: &'q Vertex) {
        if self.items.len() < CAPACITY {
            self.items.push(item);
        } else {
            if self.children.len() != 4 { self.divide(); }

            self.children
                .iter_mut()
                .find(|q| q.boundary.contains(&item))
                .unwrap()
                .insert(item);
        }
    }

    pub fn lookup(&mut self, x: f32, y: f32, r: f32, items_in_area: &mut Vec<&'q Vertex>) {
        if self.boundary.overlaps(x, y, r) {
            self.boundary.been_searched = self.draw_searched;

            self.items
                .iter()
                .filter(|v| (v.position.x - x).powf(2.) + (v.position.y - y).powf(2.) <= r.powf(2.))
                .for_each(|v| items_in_area.push(v));

            self.children
                .iter_mut()
                .for_each(|q| q.lookup(x, y, r, items_in_area));
        }
    }

    fn divide(&mut self) {
        let centre_x = (self.boundary.right_x + self.boundary.left_x) / 2.;
        let centre_y = (self.boundary.bottom_y + self.boundary.top_y) / 2.;

        // top left
        self.children.push(Self::new_with_boundary(
            QuadTreeBoundary::new(
                self.boundary.left_x,
                self.boundary.top_y,
                centre_x,
                centre_y,
            ),
            self.draw_searched,
        ));
        // top right
        self.children.push(Self::new_with_boundary(
            QuadTreeBoundary::new(
                centre_x,
                self.boundary.top_y,
                self.boundary.right_x,
                centre_y,
            ),
            self.draw_searched,
        ));
        // bottom left
        self.children.push(Self::new_with_boundary(
            QuadTreeBoundary::new(
                self.boundary.left_x,
                centre_y,
                centre_x,
                self.boundary.bottom_y,
            ),
            self.draw_searched,
        ));
        // bottom right
        self.children.push(Self::new_with_boundary(
            QuadTreeBoundary::new(
                centre_x,
                centre_y,
                self.boundary.right_x,
                self.boundary.bottom_y,
            ),
            self.draw_searched,
        ));
    }
}

impl<'q> Drawable for QuadTree<'q> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        if self.children.len() == 4 {
            self.boundary.draw(target, states);
            self.children.iter().for_each(|q| q.draw(target, states));
        }
    }
}