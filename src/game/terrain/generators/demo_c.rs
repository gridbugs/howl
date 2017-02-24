use ecs::*;
use game::*;
use game::data::*;
use game::terrain::util;
use coord::Coord;

const STAIR_COORDS: [Coord; 2] = [
    Coord { x: 7, y: 9 },
    Coord { x: 22, y: 17 },
];

const START_COORD: Coord = STAIR_COORDS[0];

pub fn demo_c<S: TurnScheduleQueue>(ids: &EntityIdReserver,
                                  rng: &GameRng,
                                  schedule: &mut S,
                                  g: &mut EcsAction,
                                  parent: ParentLevelCtx) -> TerrainMetadata {

    let (width, height) = util::terrain_from_strings(&level_str(), None, ids, schedule, g);

    util::generate_clouds(width, height, ids, rng, schedule, g);

    let mut connections = LevelConnectionReport::new();
    let mut count = 0;

    if let Some(entrance_group) = parent.level.ecs.level_switch_group(parent.entrance_entity_id) {

        // loop through all level switches with groups in the parent level
        for (id, group) in parent.level.ecs.level_switch_group_iter() {

            // check if the group is the same as the group of the current level switch
            if group == entrance_group {

                // create corresponding up stairs
                let local_id = ids.new_id();

                let existing = ExistingLevel {
                    level_id: parent.level_id,
                    entrance_entity_id: id,
                };

                prototypes::up_stairs(g.entity_mut(local_id),
                                      STAIR_COORDS[count],
                                      LevelSwitch::ExistingLevel(existing),
                                      Some(count));

                // the group is used to distinguish between stairs later
                count += 1;

                // connect the level switch in the parent to the new level switch
                connections.connect(id, local_id);

            }
        }
    } else {
        let local_id = ids.new_id();

        let existing = ExistingLevel {
            level_id: parent.level_id,
            entrance_entity_id: parent.entrance_entity_id,
        };

        prototypes::up_stairs(g.entity_mut(local_id),
                              STAIR_COORDS[count],
                              LevelSwitch::ExistingLevel(existing),
                              Some(count));

        connections.connect(parent.entrance_entity_id, local_id);
    }

    TerrainMetadata {
        width: width,
        height: height,
        start_coord: START_COORD,
        connection_report: connections,
    }
}


fn level_str() -> Vec<&'static str> {
    vec!["#########################################",
         "#.......................................#",
         "#.......................................#",
         "#....................######.............#",
         "#....................#..................#",
         "#....................#..................#",
         "#####.....############..................#",
         "#....................#..................#",
         "#....................#........#.........#",
         "#.............................#.........#",
         "#....................#........#.........#",
         "#....................#........#.........#",
         "#....................#........#.........#",
         "#######.##########################......#",
         "#............#...................#......#",
         "#............#...................#......#",
         "#............#..........................#",
         "#................................#......#",
         "#............#...................#......#",
         "#............#...................#......#",
         "#########################################",]
}
