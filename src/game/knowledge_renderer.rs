use std::ops::Deref;
use ecs_content::*;
use engine_defs::LevelId;
use game::*;
use math::Coord;
use message::*;
use content_types::*;

pub trait KnowledgeRenderer {

    /// Resets any internal buffers
    fn reset_buffers(&mut self);

    /// Width of game window in cells
    fn width(&self) -> usize;

    /// Height of game window in cells
    fn height(&self) -> usize;

    /// Coordinate of top-left corner cell rendered in game window in world-space
    fn world_offset(&self) -> Coord;

    /// Given a coordinate in world-space, converts it into a coordinate in screen-space
    fn world_to_screen(&self, coord: Coord) -> Coord {
        coord - self.world_offset()
    }

    /// Offset required to make given coordinate in world-space appear in the centre of
    /// the game window
    fn centre_offset(&self, centre: Coord) -> Coord {
        centre - Coord::new(self.width() as isize / 2, self.height() as isize / 2)
    }

    /// Highest coordinate in world-space that appears in the game window
    fn world_limit(&self) -> Coord {
        self.world_offset() + Coord::new(self.width() as isize - 1, self.height() as isize - 1)
    }

    /// Returns true iff the given coordinate in world-space corresponds to a cell in
    /// the game window
    fn contains_world_coord(&self, coord: Coord) -> bool {
        coord.x >= self.world_offset().x && coord.y >= self.world_offset().y &&
            coord.x < self.world_limit().x && coord.y < self.world_limit().y
    }

    /// Update the contents of internal buffer of the contents of the game window.
    /// Does not update the display.
    fn update_game_window_buffer(&mut self, knowledge: &DrawableKnowledgeLevel, turn_id: u64, position: Coord);

    /// Returns the number of lines of the log that will be displayed at a time
    fn log_num_lines(&self) -> usize;

    /// Updates the contents of internal message log with the last `self.log_num_lines()`
    /// lines of the message log, translating into the given language.
    fn update_log_buffer(&mut self, messages: &MessageLog, language: &Box<Language>);

    /// Push the currently drawn content to the physical display
    fn publish(&mut self);

    /// Updates the game window with the contents of the internal buffer
    fn draw_game_window(&mut self);

    /// Updates the log with the contents of the internal message log
    fn draw_log(&mut self);

    /// Updates the hud based on a specified entity
    fn draw_hud_bottom<E: Entity>(&mut self, entity: &E, language: &Box<Language>);
    fn draw_hud<E: Entity>(&mut self, entity: &E, language: &Box<Language>);

    /// Updates the game window with the contents of the internal buffer
    /// drawing a specified overlay over the top
    fn draw_game_window_with_overlay(&mut self, overlay: &RenderOverlay);

    /// Display a fullscreen view of message log
    fn fullscreen_log(&mut self, message_log: &MessageLog, offset: usize, language: &Box<Language>);

    /// Number of lines in fullscreen message log view
    fn fullscreen_log_num_rows(&self) -> usize;

    /// Number of characters that fit in a single line of the fullscreen message log
    fn fullscreen_log_num_cols(&self) -> usize;

    /// Displays a message in fullscreen
    fn fullscreen_message(&mut self, message_type: MessageType, language: &Box<Language>) {
        let mut message = Message::new();
        language.translate(message_type, &mut message);
        self.fullscreen_translated_message(&message, 0);
    }

    /// Wraps a message to fit in fullscreen
    fn fullscreen_wrap(&self, message: &Message, wrapped: &mut Vec<TextMessage>) {
        wrap_message(&message, self.fullscreen_log_num_cols(), wrapped);
    }

    /// Displays a translated message in fullscreen
    fn fullscreen_translated_message(&mut self, message: &Message, offset: usize) {
        let mut wrapped = Vec::new();
        self.fullscreen_wrap(message, &mut wrapped);
        self.fullscreen_wrapped_translated_message(&wrapped, offset);
    }

    /// Displays a wrapped, translated message in fullscreen
    fn fullscreen_wrapped_translated_message(&mut self, wrapped: &Vec<TextMessage>, offset: usize);

    /// Display a fullscreen menu
    fn fullscreen_menu<T>(&mut self, prelude: Option<MessageType>, menu: &SelectMenu<T>, state: &SelectMenuState, language: &Box<Language>);

    fn publish_game_window(&mut self) {
        self.draw_game_window();
        self.publish();
    }

    fn publish_game_window_with_overlay(&mut self, overlay: &RenderOverlay) {
        self.draw_game_window_with_overlay(overlay);
        self.publish();
    }

    fn publish_all_windows<E: Entity>(&mut self, entity: &E, language: &Box<Language>) {
        self.draw_game_window();
        self.draw_log();
        self.draw_hud(entity, language);
        self.publish();
    }

    fn publish_all_windows_with_overlay<E: Entity>(&mut self, entity: &E, language: &Box<Language>, overlay: &RenderOverlay) {
        self.draw_game_window_with_overlay(overlay);
        self.draw_log();
        self.draw_hud(entity, language);
        self.publish();
    }

    fn update_and_publish_game_window(&mut self,
                                      turn_id: u64,
                                      knowledge: &DrawableKnowledgeLevel,
                                      position: Coord) {
        self.update_game_window_buffer(knowledge, turn_id, position);
        self.draw_game_window();
        self.publish();
    }


    fn update_and_publish_all_windows<E: Entity>(&mut self,
                                                 turn_id: u64,
                                                 knowledge: &DrawableKnowledgeLevel,
                                                 position: Coord,
                                                 messages: &MessageLog,
                                                 entity: &E,
                                                 language: &Box<Language>) {

        self.update_log_buffer(messages, language);
        self.update_game_window_buffer(knowledge, turn_id, position);

        self.draw_game_window();
        self.draw_log();
        self.draw_hud(entity, language);

        self.publish();
    }

    fn update_and_publish_all_windows_with_overlay<E: Entity>(&mut self,
                                                              turn_id: u64,
                                                              knowledge: &DrawableKnowledgeLevel,
                                                              position: Coord,
                                                              messages: &MessageLog,
                                                              entity: &E,
                                                              language: &Box<Language>,
                                                              overlay: &RenderOverlay) {

        self.update_log_buffer(messages, language);
        self.update_game_window_buffer(knowledge, turn_id, position);

        self.draw_game_window_with_overlay(overlay);
        self.draw_log();
        self.draw_hud(entity, language);

        self.publish();

    }



    fn update_and_publish_all_windows_for_entity<E: Entity>(&mut self,
                                                            turn_id: u64,
                                                            level_id: LevelId,
                                                            entity: &E,
                                                            language: &Box<Language>) {
        let knowledge = entity.borrow_drawable_knowledge().expect("Expected drawable_knowledge component");
        let knowledge_level = knowledge.level(level_id);
        self.update_and_publish_all_windows(turn_id,
                                            knowledge_level,
                                            entity.copy_position().expect("Expected position component"),
                                            entity.borrow_message_log().expect("Expected message_log component").deref(),
                                            entity,
                                            language);
    }

    fn update_and_publish_all_windows_for_entity_with_overlay<E: Entity>(&mut self,
                                                                         turn_id: u64,
                                                                         level_id: LevelId,
                                                                         entity: &E,
                                                                         language: &Box<Language>,
                                                                         overlay: &RenderOverlay) {
        let knowledge = entity.borrow_drawable_knowledge().expect("Expected drawable_knowledge component");
        let knowledge_level = knowledge.level(level_id);
        self.update_and_publish_all_windows_with_overlay(turn_id,
                                                         knowledge_level,
                                                         entity.copy_position().expect("Expected position component"),
                                                         entity.borrow_message_log().expect("Expected message_log component").deref(),
                                                         entity,
                                                         language,
                                                         overlay);
    }

    fn publish_fullscreen_menu<T>(&mut self, prelude: Option<MessageType>, menu: &SelectMenu<T>, state: &SelectMenuState, language: &Box<Language>) {
        self.fullscreen_menu(prelude, menu, state, language);
        self.publish();
    }

    fn publish_fullscreen_menu_with_hud<T, E: Entity>(&mut self, prelude: Option<MessageType>, menu: &SelectMenu<T>, state: &SelectMenuState,
                                           language: &Box<Language>, entity: &E) {
        self.fullscreen_menu(prelude, menu, state, language);
        self.draw_hud_bottom(entity, language);
        self.publish();
    }

    fn publish_fullscreen_log(&mut self, message_log: &MessageLog, offset: usize, language: &Box<Language>) {
        self.fullscreen_log(message_log, offset, language);
        self.publish();
    }

    fn publish_fullscreen_message(&mut self, message_type: MessageType, language: &Box<Language>) {
        self.fullscreen_message(message_type, language);
        self.publish();
    }

    fn publish_fullscreen_translated_message(&mut self, message: &Message, offset: usize) {
        self.fullscreen_translated_message(message, offset);
        self.publish();
    }
}
