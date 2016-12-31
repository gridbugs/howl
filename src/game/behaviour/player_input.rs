use game::*;
use behaviour::LeafResolution;
use direction::Direction;
use math::CoordLine;
use bresenham;

pub fn player_input(input_source: InputSourceRef) -> BehaviourLeaf {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(meta_action) = get_meta_action(input, input_source) {
                return LeafResolution::Yield(meta_action);
            }
        }
    })
}

fn get_direction(map: &ControlMap, input_source: InputSourceRef) -> Option<Direction> {
    input_source.next_input().and_then(|event| {
        map.control(event).and_then(|control| {
            control_to_direction(control)
        })
    })
}

fn control_to_direction(control: Control) -> Option<Direction> {
    match control {
        Control::Direction(d) => Some(d),
        _ => None,
    }
}

fn aim(input: BehaviourInput, map: &ControlMap, input_source: InputSourceRef) -> Option<CoordLine> {
    let start = input.entity.position().unwrap();
    let mut end = start;
    let mut overlay = RenderOverlay {
        aim_line: Some(CoordLine::new()),
    };

    loop {

        overlay.aim_line.as_mut().map(|line| {
            bresenham::make_line(start, end, line);
        });

        input.renderer.borrow_mut().draw_with_overlay(&overlay);

        if let Some(event) = input_source.next_input() {
            if let Some(control) = map.control(event) {
                if let Some(direction) = control_to_direction(control) {
                    end += direction.vector();
                } else {
                    input.renderer.borrow_mut().draw();
                    return None;
                }
            }
        }
    }
}

fn get_meta_action(input: BehaviourInput, input_source: InputSourceRef) -> Option<MetaAction> {
    input_source.next_input().and_then(|event| {
        input.entity.control_map().and_then(|map| {
            map.control(event).and_then(|control| {
                match control {
                    Control::Direction(d) => Some(MetaAction::ActionArgs(ActionArgs::Walk(input.entity.id(), d))),
                    Control::Close => {
                        get_direction(map, input_source).map(|d| MetaAction::ActionArgs(ActionArgs::Close(input.entity.id(), d)))
                    }
                    Control::Fire => {
                        aim(input, map, input_source);
                        None
                    }
                    Control::Wait => {
                        Some(MetaAction::ActionArgs(ActionArgs::Null))
                    }
                    Control::Quit => Some(MetaAction::External(External::Quit)),
                }
            })
        })
    })
}

