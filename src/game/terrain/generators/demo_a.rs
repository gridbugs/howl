use ecs::*;
use game::*;
use game::data::*;
use game::terrain::util;
use coord::Coord;

const START_COORD: Coord = Coord { x: 18, y: 14 };

const STAIR_COORD_A: Coord = Coord { x: 8, y: 5 };
const STAIR_COORD_B: Coord = Coord { x: 31, y: 21 };

pub fn demo_a<S: TurnScheduleQueue>(ids: &EntityIdReserver,
                                  rng: &GameRng,
                                  schedule: &mut S,
                                  g: &mut EcsAction) -> TerrainMetadata {

    let level_switch = LevelSwitch::NewLevel(TerrainType::DemoB);
    let (width, height) = util::terrain_from_strings(&level_str(), Some(level_switch), ids, schedule, g);

    util::generate_clouds(width, height, ids, rng, schedule, g);

    prototypes::down_stairs(g.entity_mut(ids.new_id()),
                            STAIR_COORD_A,
                            LevelSwitch::NewLevel(TerrainType::DemoC),
                            Some(0));

    prototypes::down_stairs(g.entity_mut(ids.new_id()),
                            STAIR_COORD_B,
                            LevelSwitch::NewLevel(TerrainType::DemoC),
                            Some(0));

    TerrainMetadata {
        width: width,
        height: height,
        start_coord: START_COORD,
        connection_report: LevelConnectionReport::new(),
    }
}

fn level_str() -> Vec<&'static str> {
    vec!["&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",
         "&,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,&",
         "&,,############################,,,,,,&",
         "&,,#.........#................#,,&,,,&",
         "&,,#.........#................#,,,&,,&",
         "&,,#..........................#,,&,,,&",
         "&&,#.........#................#,,,,,,&",
         "&,&#.........##########+#######,,,,,,&",
         "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
         "&&,#.........#,t,,,,,,,&,,,,,,,&,&,&,&",
         "&,,#.........#,,,,t&,,,,,,,,&,,,,,,,,&",
         "&,,#.........+,,,,,,&,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
         "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
         "&,&#.........#,,,,,,,=,&,,,,,,,,,,,,,&",
         "&,,###########,t,,,,,&,,,,,,,&,&,,,,,&",
         "&,,&,,,,,,,,,,,,,,,,,&,,,,&,,,,,,,,,,&",
         "&,&,,,,,,,,,,,,&,,,,,,,,,,,,,,,,,,,,,&",
         "&,,,&,,,,,,,,,,,,,,,,&,,,,,#########,&",
         "&,&,,,&,,,,,&,,&,,,,&,,,,,,#.......#,&",
         "&,,,,,&,,,,,,,,,&,,,,&,,,,,#.......#,&",
         "&,,,,,,,,,&,,,,,,,,,,,,,&,,........#,&",
         "&,&,&,,,,&&,,,&,&,,,,,,,&,,#.......#,&",
         "&,,,,,,,,,,,,,,,,,,,&,,,,,,#.......#,&",
         "&,,,&,,,,,,,&,,,,,,,,,,,,,,#########,&",
         "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",]
}
