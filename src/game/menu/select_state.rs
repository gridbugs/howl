use std::slice;
use game::*;

#[derive(Clone)]
pub struct SelectMenuState {
    selected_index: usize,
}

impl SelectMenuState {
    pub fn new() -> Self {
        Self::new_with_index(0)
    }

    pub fn new_with_index(index: usize) -> Self {
        SelectMenuState {
            selected_index: index,
        }
    }

    pub fn select_next<T>(&mut self, menu: &SelectMenu<T>) {
        self.selected_index += 1;
        if self.selected_index >= menu.len() {
            self.selected_index = 0;
        }
    }

    pub fn select_prev<T>(&mut self, menu: &SelectMenu<T>) {
        if self.selected_index == 0 {
            self.selected_index = menu.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn confirm<T>(&self, mut menu: SelectMenu<T>) -> T {

        assert!(self.selected_index < menu.len(),
                "Attempt to confirm non-existent menu item");

        // truncate the menu such that the selected item is at the end
        menu.truncate(self.selected_index + 1);

        // take the value of the final item in the result
        menu.pop().map(|i| i.to_value()).expect("Unexpected empty menu")
    }

    pub fn iter<'a, T>(&self, menu: &'a SelectMenu<T>) -> SelectMenuStateIter<'a, T> {
        SelectMenuStateIter {
            menu_iter: menu.iter(),
            current_index: 0,
            selected_index: self.selected_index,
        }
    }
}

impl Default for SelectMenuState {
    fn default() -> Self {
        SelectMenuState::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectMenuItemState {
    Selected,
    Deselected,
}

pub struct SelectMenuStateIter<'a, T: 'a> {
    menu_iter: slice::Iter<'a, SelectMenuItem<T>>,
    current_index: usize,
    selected_index: usize,
}

impl<'a, T: 'a> Iterator for SelectMenuStateIter<'a, T> {

    type Item = (SelectMenuItemState, &'a SelectMenuItem<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.menu_iter.next().map(|item| {
            let item_state = if self.current_index == self.selected_index {
                SelectMenuItemState::Selected
            } else {
                SelectMenuItemState::Deselected
            };

            self.current_index += 1;

            (item_state, item)
        })
    }
}
