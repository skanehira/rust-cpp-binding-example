use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, PartialEq, Clone, Default, Copy)]
#[repr(C)]
pub enum TodoStatus {
    #[default]
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(C)]
pub struct Todo {
    id: i32,
    title: *const c_char,
    status: TodoStatus,
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn new_todo(id: i32, title: *const c_char) -> *mut Todo {
    Box::into_raw(Box::new(Todo::new(id, title)))
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn todo_free(o: *mut Todo) {
    if !o.is_null() {
        unsafe {
            let _ = Box::from_raw(o);
        }
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = Box::from_raw(s);
        }
    }
}

impl Todo {
    /// # Safety
    pub unsafe fn new(id: i32, title: *const c_char) -> Self {
        Todo {
            id,
            title,
            status: TodoStatus::default(),
        }
    }

    #[no_mangle]
    pub extern "C" fn status_str(&self) -> *mut c_char {
        CString::new(format!("{:?}", self.status))
            .unwrap()
            .into_raw()
    }

    #[no_mangle]
    pub extern "C" fn is_completed(&self) -> bool {
        self.status == TodoStatus::Completed
    }

    #[no_mangle]
    pub extern "C" fn title(&self) -> *const c_char {
        self.title
    }

    #[no_mangle]
    pub extern "C" fn change_status(&mut self, status: TodoStatus) {
        self.status = status;
    }

    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn change_title(&mut self, title: *const c_char) {
        self.title = title
    }
}
