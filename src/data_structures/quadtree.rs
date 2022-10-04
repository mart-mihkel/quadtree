use sfml::graphics::{Color, Drawable, PrimitiveType, RenderStates, RenderTarget, Vertex};
use sfml::system::Vector2f;

const CAPACITY: usize = 5;

pub struct QuadTreeBoundary {
    left_x: f32,
    top_y: f32,
    right_x: f32,
    bottom_y: f32,
    searched: bool,
}

impl QuadTreeBoundary {
    pub fn new(left_x: f32, top_y: f32, right_x: f32, bottom_y: f32) -> Self {
        let searched = false;

        Self { left_x, top_y, right_x, bottom_y, searched }
    }

    pub fn contains(&self, point: &Vertex) -> bool {
        point.position.x >= self.left_x &&
            point.position.x <= self.right_x &&
            point.position.y >= self.top_y &&
            point.position.y <= self.bottom_y
    }

    pub fn intersects(&self, other: &QuadTreeBoundary) -> bool {
        self.left_x <= other.right_x && self.right_x >= other.left_x && self.top_y <= other.bottom_y && self.bottom_y >= other.top_y
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

        if self.searched {
            target.draw_primitives(
                &[
                    Vertex::with_pos_color(
                        Vector2f::new(self.left_x, self.top_y),
                        Color::rgba(255, 0, 0, 32),
                    ),
                    Vertex::with_pos_color(
                        Vector2f::new(self.right_x, self.top_y),
                        Color::rgba(255, 0, 0, 32),
                    ),
                    Vertex::with_pos_color(
                        Vector2f::new(self.right_x, self.bottom_y),
                        Color::rgba(255, 0, 0, 32),
                    ),
                    Vertex::with_pos_color(
                        Vector2f::new(self.left_x, self.bottom_y),
                        Color::rgba(255, 0, 0, 32),
                    ),
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

    pub fn new_with_boundary(boundary: QuadTreeBoundary, draw_searched: bool) -> Self {
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

    pub fn query(&mut self, area: &QuadTreeBoundary) -> Vec<&Vertex> {
        let mut items_in_area = Vec::new();
        self.query_with_vec(area, &mut items_in_area);

        items_in_area
    }

    fn query_with_vec(&mut self, area: &QuadTreeBoundary, items_in_area: &mut Vec<&'q Vertex>) {
        if self.boundary.intersects(area) {
            self.boundary.searched = self.draw_searched;

            self.items
                .iter()
                .filter(|v| area.contains(v))
                .for_each(|v| items_in_area.push(v));

            self.children.iter_mut().for_each(|q| q.query_with_vec(area, items_in_area));
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