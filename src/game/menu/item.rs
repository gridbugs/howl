use game::*;

#[derive(Debug)]
pub struct MenuItem<T> {
    message: MenuMessageType,
    value: T,
}

impl <T> MenuItem<T> {
    pub fn new(message: MenuMessageType, value: T) -> Self {
        MenuItem {
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

impl MenuItem<MenuMessageType> {
    pub fn simple(message: MenuMessageType) -> Self {
        Self::new(message, message)
    }
}
