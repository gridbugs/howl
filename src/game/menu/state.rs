use std::slice;
use game::*;

pub struct MenuState {
    selected_index: usize,
}

impl MenuState {
    pub fn new() -> Self {
        Self::new_with_index(0)
    }

    pub fn new_with_index(index: usize) -> Self {
        MenuState {
            selected_index: index,
        }
    }

    pub fn select_next<T>(&mut self, menu: &Menu<T>) {
        self.selected_index += 1;
        if self.selected_index >= menu.len() {
            self.selected_index = 0;
        }
    }

    pub fn select_prev<T>(&mut self, menu: &Menu<T>) {
        if self.selected_index == 0 {
            self.selected_index = menu.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn confirm<T>(&self, mut menu: Menu<T>) -> T {

        assert!(self.selected_index < menu.len(),
                "Attempt to confirm non-existent menu item");

        // truncate the menu such that the selected item is at the end
        menu.truncate(self.selected_index + 1);

        // take the value of the final item in the result
        menu.pop().map(|i| i.to_value()).expect("Unexpected empty menu")
    }

    pub fn iter<'a, T>(&self, menu: &'a Menu<T>) -> MenuStateIter<'a, T> {
        MenuStateIter {
            menu_iter: menu.iter(),
            current_index: 0,
            selected_index: self.selected_index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemState {
    Selected,
    Deselected,
}

pub struct MenuStateIter<'a, T: 'a> {
    menu_iter: slice::Iter<'a, MenuItem<T>>,
    current_index: usize,
    selected_index: usize,
}

impl<'a, T: 'a> Iterator for MenuStateIter<'a, T> {

    type Item = (MenuItemState, &'a MenuItem<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.menu_iter.next().map(|item| {
            let item_state = if self.current_index == self.selected_index {
                MenuItemState::Selected
            } else {
                MenuItemState::Deselected
            };

            self.current_index += 1;

            (item_state, item)
        })
    }
}
