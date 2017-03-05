use game::*;

pub struct SelectMenuOperation<'a, 'b, 'c, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T> {
    renderer: &'a mut R,
    input: &'b mut I,
    prelude: Option<MessageType>,
    language: &'c Box<Language>,
    menu: SelectMenu<T>,
    initial_state: Option<SelectMenuState>,
}

impl<'a, 'b, 'c, R: 'a + KnowledgeRenderer, I: 'b + InputSource, T> SelectMenuOperation<'a, 'b, 'c, R, I, T> {
    pub fn new(renderer: &'a mut R,
               input: &'b mut I,
               prelude: Option<MessageType>,
               language: &'c Box<Language>,
               menu: SelectMenu<T>,
               initial_state: Option<SelectMenuState>) -> Self {
        SelectMenuOperation {
            renderer: renderer,
            input: input,
            prelude: prelude,
            language: language,
            menu: menu,
            initial_state: initial_state,
        }
    }

    pub fn run(self) -> (T, SelectMenuState) {
        let mut state = self.initial_state.unwrap_or_default();

        loop {
            self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);

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
            self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);

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
        self.renderer.publish_fullscreen_menu(self.prelude, &self.menu, &state, self.language);
    }
}
