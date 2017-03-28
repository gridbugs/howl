use game::*;
use message::*;


pub struct RendererBuffers {
    pub tiles: TileBuffer,
    pub message_log: Vec<Message>,
}

impl RendererBuffers {
    pub fn new(width: usize, height: usize, num_lines: usize) -> Self {
        let mut message_log = Vec::new();
        for _ in 0..num_lines {
            message_log.push(Message::new());
        }

        RendererBuffers {
            tiles: TileBuffer::new(width, height),
            message_log: message_log,
        }
    }

    pub fn reset(&mut self) {
        self.tiles.reset();
        for message in self.message_log.iter_mut() {
            message.clear();
        }
    }
}
