use control::*;
use game::*;
use ecs_content::*;
use message::*;

pub struct SelectMenuOperation<'a, 'b, 'c, 'd, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T> {
    renderer: &'a mut R,
    input: &'b mut I,
    prelude: Option<MessageType>,
    language: &'c Box<Language>,
    menu: SelectMenu<T>,
    initial_state: Option<SelectMenuState>,
    hud_entity: Option<EntityRef<'d>>,
}

impl<'a, 'b, 'c, 'd, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T> SelectMenuOperation<'a, 'b, 'c, 'd, R, I, T> {
    pub fn new(renderer: &'a mut R,
               input: &'b mut I,
               prelude: Option<MessageType>,
               language: &'c Box<Language>,
               menu: SelectMenu<T>,
               initial_state: Option<SelectMenuState>,
               hud_entity: Option<EntityRef<'d>>) -> Self {
        SelectMenuOperation {
            renderer: renderer,
            input: input,
            prelude: prelude,
            language: language,
            menu: menu,
            initial_state: initial_state,
            hud_entity: hud_entity,
        }
    }

    pub fn run(self) -> (T, SelectMenuState) {
        let mut state = self.initial_state.unwrap_or_default();

        loop {
            if let Some(entity) = self.hud_entity {
                self.renderer.publish_fullscreen_menu_with_hud(self.prelude, &self.menu, &state, self.language, entity);
            } else {
                self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);
            }

            if let Some(event) = self.input.next_input() {
                match event {
                    InputEvent::Down => {
                        state.select_next(&self.menu);
                    }
                    InputEvent::Up => {
                        state.select_prev(&self.menu);
                    }
                    InputEvent::Return => {
                        return (state.confirm(self.menu), state);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn run_can_escape(self) -> Option<(T, SelectMenuState)> {
        let mut state = self.initial_state.unwrap_or_default();

        loop {
            if let Some(entity) = self.hud_entity {
                self.renderer.publish_fullscreen_menu_with_hud(self.prelude, &self.menu, &state, self.language, entity);
            } else {
                self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);
            }

            if let Some(event) = self.input.next_input() {
                match event {
                    InputEvent::Down => {
                        state.select_next(&self.menu);
                    }
                    InputEvent::Up => {
                        state.select_prev(&self.menu);
                    }
                    InputEvent::Return => {
                        if self.menu.is_empty() {
                            return None;
                        } else {
                            return Some((state.confirm(self.menu), state));
                        }
                    }
                    InputEvent::Escape => {
                        return None;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn publish(self) {
        let state = self.initial_state.unwrap_or_default();
        if let Some(entity) = self.hud_entity {
            self.renderer.publish_fullscreen_menu_with_hud(self.prelude, &self.menu, &state, self.language, entity);
        } else {
            self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);
        }
    }
}
