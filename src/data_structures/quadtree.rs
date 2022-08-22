use sfml::graphics::{Drawable, PrimitiveType, RenderStates, RenderTarget, Vertex};
use sfml::system::Vector2f;

const CAPACITY: usize = 5;

pub struct QuadTreeBoundary {
    top_left_x: f32,
    top_left_y: f32,
    bottom_right_x: f32,
    bottom_right_y: f32,
}

impl QuadTreeBoundary {
    pub fn new(top_left_x: f32, top_left_y: f32, bottom_right_x: f32, bottom_right_y: f32) -> Self {
        Self { top_left_x, top_left_y, bottom_right_x, bottom_right_y }
    }

    pub fn contains(&self, point: &Vertex) -> bool {
        point.position.x >= self.top_left_x &&
            point.position.x <= self.bottom_right_x &&
            point.position.y >= self.top_left_y &&
            point.position.y <= self.bottom_right_y
    }

    pub fn intersects(&self, other: &QuadTreeBoundary) -> bool {
        self.top_left_x <= other.bottom_right_x ||
            self.bottom_right_x >= other.top_left_x ||
            self.top_left_y >= other.bottom_right_y ||
            self.bottom_right_y <= other.top_left_y
    }
}

impl Drawable for QuadTreeBoundary {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let centre_x = (self.bottom_right_x + self.top_left_x) / 2.;
        let centre_y = (self.bottom_right_y + self.top_left_y) / 2.;

        target.draw_primitives(
            &[
                Vertex::with_pos(Vector2f::new(self.top_left_x, centre_y)),
                Vertex::with_pos(Vector2f::new(self.bottom_right_x, centre_y)),
                Vertex::with_pos(Vector2f::new(centre_x, self.top_left_y)),
                Vertex::with_pos(Vector2f::new(centre_x, self.bottom_right_y)),
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
}

impl<'q> QuadTree<'q> {
    pub fn new(width: f32, height: f32) -> Self {
        Self::new_with_boundary(QuadTreeBoundary::new(0., 0., width, height))
    }

    pub fn new_with_boundary(boundary: QuadTreeBoundary) -> Self {
        let items = Vec::with_capacity(CAPACITY);
        let children = Vec::with_capacity(4);

        Self { items, children, boundary }
    }

    pub fn insert(&mut self, item: &'q Vertex) {
        if self.items.len() < CAPACITY {
            self.items.push(item);
        } else {
            if self.children.len() != 4 {
                self.divide();
            }

            self.children
                .iter_mut()
                .find(|q| q.boundary.contains(&item))
                .unwrap()
                .insert(item);
        }
    }

    pub fn query(&self, area: &QuadTreeBoundary) -> Vec<&Vertex> {
        let mut items_in_area = Vec::new();
        self.query_with_vec(area, &mut items_in_area);

        items_in_area
    }

    fn query_with_vec(&self, area: &QuadTreeBoundary, items_in_area: &mut Vec<&'q Vertex>) {
        if self.boundary.intersects(area) {
            self.items
                .iter()
                .filter(|v| area.contains(v))
                .for_each(|v| items_in_area.push(v));

            self.children.iter().for_each(|q| q.query_with_vec(area, items_in_area));
        }
    }

    fn divide(&mut self) {
        let centre_x = (self.boundary.bottom_right_x + self.boundary.top_left_x) / 2.;
        let centre_y = (self.boundary.bottom_right_y + self.boundary.top_left_y) / 2.;

        // top left
        self.children.push(Self::new_with_boundary(QuadTreeBoundary::new(
            self.boundary.top_left_x,
            self.boundary.top_left_y,
            centre_x,
            centre_y,
        )));
        // top right
        self.children.push(Self::new_with_boundary(QuadTreeBoundary::new(
            centre_x,
            self.boundary.top_left_y,
            self.boundary.bottom_right_x,
            centre_y,
        )));
        // bottom left
        self.children.push(Self::new_with_boundary(QuadTreeBoundary::new(
            self.boundary.top_left_x,
            centre_y,
            centre_x,
            self.boundary.bottom_right_y,
        )));
        // bottom right
        self.children.push(Self::new_with_boundary(QuadTreeBoundary::new(
            centre_x,
            centre_y,
            self.boundary.bottom_right_x,
            self.boundary.bottom_right_y,
        )));
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