use control::*;
use game::*;
use ecs_content::*;
use message::*;

pub struct SelectMenuOperation<'a, 'b, 'c, 'd, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T, E: 'd + Entity> {
    renderer: &'a mut R,
    input: &'b mut I,
    prelude: Option<MessageType>,
    language: &'c Box<Language>,
    menu: SelectMenu<T>,
    initial_state: Option<SelectMenuState>,
    hud_entity: Option<&'d E>,
}

impl<'a, 'b, 'c, 'd, 'e, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T> SelectMenuOperation<'a, 'b, 'c, 'd, R, I, T, EntityRef<'e>> {
    pub fn new_no_hud(renderer: &'a mut R,
                      input: &'b mut I,
                      prelude: Option<MessageType>,
                      language: &'c Box<Language>,
                      menu: SelectMenu<T>,
                      initial_state: Option<SelectMenuState>) -> Self {
        Self::new(renderer, input, prelude, language, menu, initial_state, None)
    }
}

impl<'a, 'b, 'c, 'd, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T, E: Entity> SelectMenuOperation<'a, 'b, 'c, 'd, R, I, T, E> {
    pub fn new(renderer: &'a mut R,
               input: &'b mut I,
               prelude: Option<MessageType>,
               language: &'c Box<Language>,
               menu: SelectMenu<T>,
               initial_state: Option<SelectMenuState>,
               hud_entity: Option<&'d E>) -> Self {
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

            match self.input.next_input() {
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

    pub fn run_can_escape(self) -> Option<(T, SelectMenuState)> {
        let mut state = self.initial_state.unwrap_or_default();

        loop {
            if let Some(entity) = self.hud_entity {
                self.renderer.publish_fullscreen_menu_with_hud(self.prelude, &self.menu, &state, self.language, entity);
            } else {
                self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);
            }

            match self.input.next_input() {
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

    pub fn publish(self) {
        let state = self.initial_state.unwrap_or_default();
        if let Some(entity) = self.hud_entity {
            self.renderer.publish_fullscreen_menu_with_hud(self.prelude, &self.menu, &state, self.language, entity);
        } else {
            self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);
        }
    }
}
