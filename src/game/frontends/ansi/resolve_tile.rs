use game::*;
use frontends::ansi;

pub fn resolve_tile(tile_type: TileType) -> ansi::ComplexTile {
    match tile_type {
        TileType::Wall => ansi::ComplexTile::Wall {
            front: ansi::SimpleTile::Full {
                ch: '▄',
                fg: ansi::colours::YELLOW,
                bg: ansi::colours::GREY,
                style: ansi::styles::NONE,
            },
            back: ansi::SimpleTile::Foreground('█', ansi::colours::GREY,
                                               ansi::styles::NONE),
        },
        TileType::Tree => ansi::foreground('&', ansi::colours::GREEN,
                                           ansi::styles::NONE),
        TileType::DeadTree => ansi::foreground('£', ansi::colours::YELLOW,
                                               ansi::styles::BOLD),
        TileType::Floor => ansi::full('.', ansi::colours::WHITE,
                                      ansi::colours::DARK_GREY,
                                      ansi::styles::NONE),
        TileType::Ground => ansi::full('.', ansi::colours::WHITE,
                                       ansi::colours::DARK_GREY,
                                       ansi::styles::NONE),
        TileType::OpenDoor => ansi::full('-', ansi::colours::WHITE,
                                         ansi::colours::DARK_GREY,
                                         ansi::styles::NONE),
        TileType::ClosedDoor => ansi::full('+', ansi::colours::WHITE,
                                           ansi::colours::DARK_GREY,
                                           ansi::styles::NONE),
        TileType::Bullet => ansi::foreground('*', ansi::colours::RED,
                                             ansi::styles::NONE),
        TileType::Player => ansi::foreground('@', ansi::colours::WHITE,
                                             ansi::styles::BOLD),
        TileType::TerrorPillar => ansi::foreground('t', ansi::colours::GREEN,
                                                   ansi::styles::BOLD),
        TileType::TerrorFly => ansi::foreground('T', ansi::colours::GREEN,
                                                ansi::styles::BOLD),
    }
}
