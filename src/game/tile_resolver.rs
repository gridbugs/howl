use game::TileType;

pub trait TileResolver {
    type Tile;
    fn resolve(&self, tile_type: TileType) -> Self::Tile;
}
