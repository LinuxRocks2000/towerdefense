pub mod shape;
pub mod position;
use shape::Shape;
use position::Position;

pub struct PhysicsObject {
    shape : Shape,
    position : Position
}