use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{WPARAM, LPARAM};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::Font;
use super::ControlHandle;
use std::cell::{Ref, RefMut, RefCell};
use std::fmt::Display;
use std::mem;

const NOT_BOUND: &'static str = "ListBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ListBox handle is not HWND!";

/**
A list box is a control window that contains a simple list of items from which the user can choose.
*/
#[derive(Default, Debug)]
pub struct ListBox<D: Display+Default> {
    pub handle: ControlHandle,
    collection: RefCell<Vec<D>>
}

impl<D: Display+Default> ListBox<D> {


    /// Add a new item to the listbox. Sort the collection if the listbox is sorted.
    pub fn push(&self, item: D) {
        use winapi::um::winuser::LB_ADDSTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let display = format!("{}", item);
        let display_os = to_utf16(&display);

        unsafe {
            wh::send_message(handle, LB_ADDSTRING, 0, mem::transmute(display_os.as_ptr()));
        }

        self.collection.borrow_mut().push(item);
    }


    /// Remove the item at the selected index and returns it.
    /// Panic of the index is out of bounds
    pub fn remove(&self, index: usize) -> D {
        use winapi::um::winuser::LB_DELETESTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, LB_DELETESTRING, index as WPARAM, 0);

        let mut col_ref = self.collection.borrow_mut();
        col_ref.remove(index)
    }

    /// Return the index of the currencty selected item for single value list box.
    /// Return `None` if no item is selected.
    pub fn selection(&self) -> Option<usize> {
        use winapi::um::winuser::{LB_GETCURSEL , CB_ERR};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let index = wh::send_message(handle, LB_GETCURSEL , 0, 0);

        if index == CB_ERR { None }
        else { Some(index as usize) }
    }

    /// Return the display value of the currenctly selected item for single value
    /// Return `None` if no item is selected. This reads the visual value.
    pub fn selection_string(&self) -> Option<String> {
        use winapi::um::winuser::{LB_GETCURSEL, LB_GETTEXTLEN, LB_GETTEXT, CB_ERR};
        use winapi::shared::ntdef::WCHAR;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let index = wh::send_message(handle, LB_GETCURSEL, 0, 0);

        if index == CB_ERR { None }
        else {
            let index = index as usize;
            let length = wh::send_message(handle, LB_GETTEXTLEN, index, 0) as usize;
            let mut buffer: Vec<WCHAR> = Vec::with_capacity(length);
            unsafe { 
                buffer.set_len(length); 
                wh::send_message(handle, LB_GETTEXT, index, mem::transmute(buffer.as_ptr()));
            }

            Some(from_utf16(&buffer))
        }
    }

    /// Set the currently selected item in the list box.
    /// Does nothing if the index is out of bound
    /// If the value is None, remove the selected value
    pub fn set_selection(&self, index: Option<usize>) {
        use winapi::um::winuser::LB_SETCURSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let index = index.unwrap_or(-1isize as usize);
        wh::send_message(handle, LB_SETCURSEL, index, 0);
    }

    /// Search an item that begins by the value and select the first one found.
    /// The search is not case sensitive, so this string can contain any combination of uppercase and lowercase letters.
    /// Return the index of the selected string or None if the search was not successful
    pub fn set_selection_string(&self, value: &str) -> Option<usize> {
        use winapi::um::winuser::{LB_SELECTSTRING, CB_ERR};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let os_string = to_utf16(value);

        unsafe {
            let index = wh::send_message(handle, LB_SELECTSTRING, 0, mem::transmute(os_string.as_ptr()));
            if index == CB_ERR {
                None
            } else {
                Some(index as usize)
            }
        }
    }

    /// Update the visual of the control with the inner collection.
    /// This rebuild every item in the list box and can take some time on big collections.
    pub fn sync(&self) {
        use winapi::um::winuser::{LB_ADDSTRING, LB_INITSTORAGE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        self.clear_inner(handle);

        let item_count = self.collection.borrow().len();
        wh::send_message(handle, LB_INITSTORAGE, item_count as WPARAM, (10*item_count) as LPARAM);

        for item in self.collection.borrow().iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            
            unsafe {
                wh::send_message(handle, LB_ADDSTRING, 0, mem::transmute(display_os.as_ptr()));
            }
        }
    }

    /// Set the item collection of the list box. Return the old collection
    pub fn set_collection(&self, mut col: Vec<D>) -> Vec<D> {
        use winapi::um::winuser::LB_ADDSTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        self.clear_inner(handle);

        for item in col.iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            
            unsafe {
                wh::send_message(handle, LB_ADDSTRING, 0, mem::transmute(display_os.as_ptr()));
            }
        }

        let mut col_ref = self.collection.borrow_mut();
        mem::swap::<Vec<D>>(&mut col_ref, &mut col);

        col
    }

    /// Return the number of items in the control. NOT the inner rust collection
    pub fn len(&self) -> usize {
        use winapi::um::winuser::LB_GETCOUNT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let count = wh::send_message(handle, LB_GETCOUNT, 0, 0);

        count as usize
    }


    //
    // Common control functions
    //

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Get read-only access to the inner collection of the list box
    /// This call refcell.borrow under the hood. Be sure to drop the value before
    /// calling other list box methods
    pub fn collection(&self) -> Ref<Vec<D>> {
        self.collection.borrow()
    }

    /// Get mutable access to the inner collection of the list box. Does not update the visual
    /// control. Call `sync` to update the view. This call refcell.borrow_mut under the hood. 
    /// Be sure to drop the value before calling other list box methods
    pub fn collection_mut(&self) -> RefMut<Vec<D>> {
        self.collection.borrow_mut()
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("ListBox")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {
        (::winapi::um::winuser::WS_VISIBLE, 0)
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{LBS_HASSTRINGS, WS_BORDER, WS_VSCROLL, LBS_NOTIFY, WS_CHILD};

        LBS_HASSTRINGS | LBS_NOTIFY | WS_BORDER  | WS_CHILD | WS_VSCROLL
    }

    /// Remove all value displayed in the control without touching the rust collection
    fn clear_inner(&self, handle: HWND) {
        use winapi::um::winuser::LB_RESETCONTENT;
        wh::send_message(handle, LB_RESETCONTENT, 0, 0);
    }

}
