use gm::flat::Shape;
use rapier2d::{
    math::{Point, Real},
    prelude::ColliderBuilder,
};

pub trait ToCollider {
    fn make_collider(&self) -> ColliderBuilder;
}

impl ToCollider for Shape {
    fn make_collider(&self) -> ColliderBuilder {
        match self {
            Shape::Rect(size) => ColliderBuilder::cuboid(size.width, size.height),
            Shape::Circle(r) => ColliderBuilder::ball(*r),
            Shape::Triangle(a, b, c) => {
                ColliderBuilder::triangle([a.x, a.y].into(), [b.x, b.y].into(), [c.x, c.y].into())
            }
            Shape::Polygon(points) => polygon_collider(points),
        }
    }
}

fn polygon_collider(points: &[gm::flat::Point]) -> ColliderBuilder {
    let points: Vec<_> = points.iter().map(|p| Point::<Real>::new(p.x, p.y)).collect();
    let indices: Vec<_> = (0..u32::try_from(points.len()).unwrap() - 1)
        .map(|i| [i, i + 1])
        .chain([[u32::try_from(points.len()).unwrap() - 1, 0]])
        .collect();

    ColliderBuilder::convex_decomposition(&points, &indices)
}