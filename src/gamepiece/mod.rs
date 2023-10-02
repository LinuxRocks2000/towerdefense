use crate::physics::shape::Shape;
use crate::physics::PhysicsObject;


trait GamePieceController {
    fn request_default_dimensions(&self) -> Shape;
}


pub struct GamePiece {
    controller : Box<dyn GamePieceController>,
    physics : PhysicsObject
}