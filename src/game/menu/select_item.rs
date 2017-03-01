use game::*;

#[derive(Debug)]
pub struct SelectMenuItem<T> {
    message: MenuMessageType,
    value: T,
}

impl <T> SelectMenuItem<T> {
    pub fn new(message: MenuMessageType, value: T) -> Self {
        SelectMenuItem {
            message: message,
            value: value,
        }
    }

    pub fn message(&self) -> MenuMessageType {
        self.message
    }

    pub fn to_value(self) -> T {
        self.value
    }
}

impl SelectMenuItem<MenuMessageType> {
    pub fn simple(message: MenuMessageType) -> Self {
        Self::new(message, message)
    }
}
