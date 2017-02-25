use ecs::*;
use game::*;
use game::data::*;
use game::terrain::util;
use coord::Coord;

const STAIR_COORDS: [Coord; 2] = [
    Coord { x: 7, y: 9 },
    Coord { x: 22, y: 17 },
];

pub fn demo_c<S: TurnScheduleQueue>(ids: &EntityIdReserver,
                                  rng: &GameRng,
                                  schedule: &mut S,
                                  g: &mut EcsAction,
                                  parent: ParentLevelCtx) -> TerrainMetadata {

    let (width, height) = util::terrain_from_strings(&level_str(), None, ids, schedule, g);

    util::generate_clouds(width, height, ids, rng, schedule, g);

    let mut connections = LevelConnectionReport::new();
    let mut count = 0;

    let mut start_coord = None;

    if let Some(entrance_group) = parent.level.ecs.level_switch_group(parent.exit_id) {

        // loop through all level switches with groups in the parent level
        for (id, group) in parent.level.ecs.level_switch_group_iter() {

            // check if the group is the same as the group of the current level switch
            if group == entrance_group {

                if id == parent.exit_id {
                    start_coord = Some(STAIR_COORDS[count]);
                }

                // create corresponding up stairs
                let local_id = ids.new_id();

                let existing = LevelExit {
                    level_id: parent.level_id,
                    exit_id: id,
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

        start_coord = Some(STAIR_COORDS[count]);

        let local_id = ids.new_id();

        let existing = LevelExit {
            level_id: parent.level_id,
            exit_id: parent.exit_id,
        };

        prototypes::up_stairs(g.entity_mut(local_id),
                              STAIR_COORDS[count],
                              LevelSwitch::ExistingLevel(existing),
                              Some(count));

        connections.connect(parent.exit_id, local_id);
    }

    TerrainMetadata {
        width: width,
        height: height,
        start_coord: start_coord.expect("Expected start coordinate"),
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
