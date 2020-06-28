//! All the events that can be dispatched by the built-in controls of native-windows-gui


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MousePressEvent {
    MousePressLeftUp,
    MousePressLeftDown,
    MousePressRightUp,
    MousePressRightDown
}

/// Events are identifier that are sent by controls on user interaction
/// Some events also have data that can be further processed by the event loop. See `EventData`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Event {
    /// Undefined / not implemented event. This can be dispatched by the bigger controls such as ListView and TreeView
    Unknown,

    /// Generic mouse press events that can be generated by most window controls
    OnMousePress(MousePressEvent),

    /// Generic mouse move event that can be generated by most window controls
    OnMouseMove,

    /// Generic mouse wheel event that be generated by most window controls
    /// Read the delta value with `EventData::OnMouseWheel` to check which key.
    OnMouseWheel,

    /// Generic window event when the user right click a window
    OnContextMenu,

    /// When a top level window control is created.
    OnInit,

    /// When a control needs to be redrawn
    OnPaint,

    /// When a key is pressed on a keyboard. Unlike OnKeyDown, this returns a char (ex: 'c') in a EventData::OnChar.
    OnChar,

    /// When a key is pressed on a keyboard. Use `EventData::OnKey` to check which key.
    OnKeyPress,

    /// When a key is pressed on a keyboard.Use EventData::OnKey to check which key.
    OnKeyRelease,
    
    /// When a control is resized by the user. 
    /// This is typically applied to top level windows but it also applies to children when layouts are used.
    OnResize,

    /// When a control is about to be resized by the user. 
    /// This does not triggers on maximize
    OnResizeBegin,

    /// When a control stops being resized
    /// This does not triggers on maximize
    OnResizeEnd,

    // When a window control is maximized
    OnWindowMaximize,

    // When a window control is minimized
    OnWindowMinimize,

    /// When a control is moved by the user. This is typically applied to top level windows.
    /// This is typically applied to top level windows but it also applies to children when layouts are used.
    OnMove,

    /// When a bar like control value is changed.
    OnVerticalScroll,

    /// When a bar like control value is changed.
    OnHorizontalScroll,

    /// When a file is dropped into a a control
    OnFileDrop,

    /// When a button is clicked. Similar to a MouseUp event, but only for button control
    OnButtonClick,

    /// When a button is clicked twice rapidly
    OnButtonDoubleClick,

    /// When a label is clicked
    OnLabelClick,

    /// When a label is clicked twice rapidly
    OnLabelDoubleClick,

    /// When a ImageFrame is clicked
    OnImageFrameClick,

    /// When a ImageFrame is clicked twice rapidly
    OnImageFrameDoubleClick,

    /// When a TextInput value is changed
    OnTextInput,

    /// When the list of a combobox is closed
    OnComboBoxClosed,

    /// When the list of a combobox is about to be visible
    OnComboBoxDropdown,

    /// When the current selection of the combobox was changed
    OnComboxBoxSelection,

    /// When the date select dropdown is expanded
    OnDatePickerDropdown,

    /// When the date select dropdown is closed
    OnDatePickerClosed,

    /// When the value of the date select is changed
    OnDatePickerChanged,

    /// When an item on a list box is clicked twice
    OnListBoxDoubleClick,

    /// When an item on a list box is selected
    OnListBoxSelect,

    /// The select tab of a TabsContainer changed
    TabsContainerChanged,

    /// The select tab of a TabsContainer is about to be changed
    TabsContainerChanging,

    /// When the trackbar thumb is released by the user
    TrackBarUpdated,

    /// When a menu control is opened
    OnMenuOpen,

    /// When a menu is hovered (either through mouse or keyboard)
    OnMenuHover,

    /// When the user selects on a menu item
    OnMenuItemSelected,

    /// When the user hovers over a callback tooltip
    /// The callback will also receive a `EventData::OnTooltipText`
    OnTooltipText,

    /// When the user has clicked the left mouse button within the control.
    OnTreeViewClick,

    /// When the user has clicked the left mouse button within the control twice rapidly.
    OnTreeViewDoubleClick,

    /// When the user has clicked the right mouse button within the control.
    OnTreeViewRightClick,

    /// When the control has lost the input focus
    OnTreeFocusLost,

    /// When the control has acquired the input focus
    OnTreeFocus,

    /// When an item is removed from the treeview. The item being deleted is passed in `EventData::OnTreeItemDelete`
    OnTreeItemDelete,

    /// When an item is expanded. Generates a `EventData::OnTreeItemDelete`
    OnTreeItemExpanded,

    /// When the state of a tree item is changed.
    OnTreeItemChanged,

    /// When the selected tree item is changed.
    OnTreeItemSelectionChanged,

    /// When a TrayNotification info popup (not the tooltip) is shown 
    OnTrayNotificationShow,

    /// When a TrayNotification info popup (not the tooltip) is hidden 
    OnTrayNotificationHide,

    /// When a TrayNotification is closed due to a timeout
    OnTrayNotificationTimeout,

    /// When a TrayNotification is closed due to a user click
    OnTrayNotificationUserClose,

    /// When a timer delay is elapsed
    OnTimerTick,

    /// When a notice is... noticed
    OnNotice,

    /// When a user click on the X button of a window
    OnWindowClose,
}


/// Events data sent by the controls. 
#[derive(Debug)]
pub enum EventData {
    /// The event has no data
    NoData,

    /// Sets if the window should be closed after the event
    OnWindowClose(WindowCloseData),

    /// Sets the text of a tooltip.
    /// The method `on_tooltip_text` should be used to access the inner data
    OnTooltipText(ToolTipTextData),

    /// The character inputted by a user by a `OnChar` event
    OnChar(char),

    /// The windows key code inputted by a user. See the `nwg::keys` module
    OnKey(u32),

    /// Hold resources that will most likely be used during painting. 
    OnPaint(PaintData),

    /// The delta value of a mouse wheel event. A positive value indicates that the wheel was rotated to the right; 
    /// a negative value indicates that the wheel was rotated to the left.
    OnMouseWheel(i32),

    /// The path to a file that was dropping in the application
    OnFileDrop(DropFiles),

    /// The handle to the item being deleted. The item is still valid.
    #[cfg(feature="tree-view")]
    OnTreeItemDelete(crate::TreeItem),

    /// The handle to the item being changed.
    #[cfg(feature="tree-view")]
    OnTreeItemUpdate{ item: crate::TreeItem, action: crate::TreeItemAction },

    /// The handles the the old item and the new item.
    #[cfg(feature="tree-view")]
    OnTreeItemSelectionChanged{ old: crate::TreeItem, new: crate::TreeItem },

}

impl EventData {

    /// Unwraps event data into a `&PaintData`. Panics if it's not the right type.
    pub fn on_paint(&self) -> &PaintData {
        match self {
            EventData::OnPaint(p) => p,
            d => panic!("Wrong data type: {:?}", d)
        }
    }

    /// Unwraps event data into a `&ToolTipTextData`. Panics if it's not the right type.
    pub fn on_tooltip_text(&self) -> &ToolTipTextData {
        match self {
            EventData::OnTooltipText(d) => d,
            d => panic!("Wrong data type: {:?}", d)
        }
    }

    /// Unwraps event data into a `&DragData`. Panics if it's not the right type.
    pub fn on_file_drop(&self) -> &DropFiles {
        match self {
            EventData::OnFileDrop(d) => d,
            d => panic!("Wrong data type: {:?}", d)
        }
    }

    /// Unwraps event data into the virtual key code for `OnKeyPress` and `OnKeyRelease`
    pub fn on_key(&self) -> u32 {
        match self {
            EventData::OnKey(key) => *key,
            d => panic!("Wrong data type: {:?}", d)
        }
    }

    /// uwraps event data into the removed tree item
    #[cfg(feature="tree-view")]
    pub fn on_tree_item_delete(&self) -> &crate::TreeItem {
        match self {
            EventData::OnTreeItemDelete(item) => item,
            d => panic!("Wrong data type: {:?}", d)
        }
    }

    /// uwraps event data into the update tree view item and the action
    #[cfg(feature="tree-view")]
    pub fn on_tree_item_update(&self) -> (&crate::TreeItem, crate::TreeItemAction) {
        match self {
            EventData::OnTreeItemUpdate { item, action } => (item, *action),
            d => panic!("Wrong data type: {:?}", d)
        }
    }

    /// nwraps event data into the removed tree item
    #[cfg(feature="tree-view")]
    pub fn on_tree_item_selection_changed(&self) -> (&crate::TreeItem, &crate::TreeItem) {
        match self {
            EventData::OnTreeItemSelectionChanged { old, new } => (old, new),
            d => panic!("Wrong data type: {:?}", d)
        }
    }

}

//
// Events data structures
//

use winapi::um::commctrl::NMTTDISPINFOW;
use winapi::um::winuser::{PAINTSTRUCT, BeginPaint, EndPaint};
use winapi::um::shellapi::{HDROP, DragFinish};
use winapi::shared::windef::HWND;
use std::fmt;

/// A wrapper structure that set the tooltip text on a `OnTooltipText` callback
pub struct ToolTipTextData {
    pub(crate) data: *mut NMTTDISPINFOW
}

impl ToolTipTextData {

    /// Tells the application to save the text value of the callback
    /// The `OnTooltipText` will not be called a second time for the associated control
    pub fn keep(&self, keep: bool) {
        use ::winapi::um::commctrl::TTF_DI_SETITEM;
        
        let data = unsafe { &mut *self.data };
        match keep {
            true => { data.uFlags |= TTF_DI_SETITEM; },
            false => { data.uFlags &= !TTF_DI_SETITEM; }
        }
    }

    /// Sets the text of the callback. This function will copy the text.
    /// WINAPI do not easily allow tooltip with more than 79 characters (80 with NULL)
    /// With a text > 79 characters, this method will do nothing.
    pub fn set_text<'b>(&self, text: &'b str) {
        use crate::win32::base_helper::to_utf16;
        use std::ptr;
        
        let text_len = text.len();
        if text_len > 79 {
            return;
        }

        self.clear();
        unsafe {
            let data = &mut *self.data;
            let local_text = to_utf16(text);
            ptr::copy_nonoverlapping(local_text.as_ptr(), data.szText.as_mut_ptr(), text_len);
        }
    }

    fn clear(&self) {
        use winapi::um::winnt::WCHAR;
        use std::{ptr, mem};
        
        unsafe {
            let data = &mut *self.data;
            ptr::write(&mut data.szText as *mut [WCHAR; 80], mem::zeroed());
        }
    }

}

impl fmt::Debug for ToolTipTextData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToolTipTextData")
    }
}


/// Opaque type that manage if a window should be closed after a OnClose event
pub struct WindowCloseData {
    pub(crate) data: *mut bool
}

impl WindowCloseData {

    /// Sets if the window should close after the event
    pub fn close(&self, value: bool) {
        unsafe{ *self.data = value; }
    }

    /// Returns true if the window will close after the event or false otherwise
    pub fn closing(&self) -> bool {
        unsafe{ *self.data }
    }
}

impl fmt::Debug for WindowCloseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WindowCloseData({})", self.closing())
    }
}


/// Opaque type over a paint event data
#[derive(Debug)]
pub struct PaintData {
    pub(crate) hwnd: HWND
}

impl PaintData {

    /// Wrapper over BeginPaint
    pub fn begin_paint(&self) -> PAINTSTRUCT {
        unsafe {
            let mut paint: PAINTSTRUCT = ::std::mem::zeroed();
            BeginPaint(self.hwnd, &mut paint);
            paint
        }
    }

    /// Wrapper over EndPaint
    pub fn end_paint(&self, p: &PAINTSTRUCT) {
        unsafe {
            EndPaint(self.hwnd, p);
        }
    }

}


/// Opaque type over one or more dragged files.
pub struct DropFiles {
    pub(crate) drop: HDROP,
}

impl DropFiles {

    /// Retrieves the position of the mouse pointer at the time a file was dropped during a drag-and-drop operation.
    /// The coordinates are local to the control. Ex: (0, 0) is the top left corner of the control.
    pub fn point(&self) -> [i32; 2] {
        use winapi::um::shellapi::DragQueryPoint;
        use winapi::shared::windef::POINT;

        unsafe {
            let mut pt = POINT { x: 0, y: 0 };
            DragQueryPoint(self.drop, &mut pt);
            [pt.x, pt.y]
        }
    }

    /// Return the nuber of files dropped 
    pub fn len(&self) -> usize {
        use winapi::um::shellapi::DragQueryFileW;
        use std::ptr;

        unsafe {
            DragQueryFileW(self.drop, 0xFFFFFFFF, ptr::null_mut(), 0) as usize
        }
    }

    /// Return the files path dropped into the app
    pub fn files(&self) -> Vec<String> {
        use winapi::um::shellapi::DragQueryFileW;
        use crate::win32::base_helper::from_utf16;
        use std::ptr;

        let len = self.len();
        let mut files = Vec::with_capacity(len);
        unsafe {
            for i in 0..len {
                // Need to add a +1 here for some reason
                let buffer_size = (DragQueryFileW(self.drop, i as _, ptr::null_mut(), 0) + 1) as usize;

                let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size);
                buffer.set_len(buffer_size);

                DragQueryFileW(self.drop, i as _, buffer.as_mut_ptr(), buffer_size as _);

                files.push(from_utf16(&buffer));
            }
        }

        files
    }

}

impl fmt::Debug for DropFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DragData {{ point: {:?}, files: {:?} }}", self.point(), self.files())
    }
}

impl Drop for DropFiles {

    fn drop(&mut self) {
        if !self.drop.is_null() {
            unsafe { DragFinish(self.drop) }
        }
    }

}
