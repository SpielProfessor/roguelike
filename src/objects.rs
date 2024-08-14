// An object, like an entity
// TODO: Actual implementation;
pub struct Object {
  pub x: i32,
  pub y: i32,
  pub char: String,
  pub passable: bool,
  pub discoverable: bool,
  pub id: String, // what kind of thing is this? Is it an item or an enemy? Or just an NPC?
}
