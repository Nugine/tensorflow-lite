use crate::bindings as C;

pub struct InterpreterOptions {
    options: *mut C::TfLiteInterpreterOptions,
}

unsafe impl Send for InterpreterOptions {}
unsafe impl Sync for InterpreterOptions {}

impl InterpreterOptions {
    pub(crate) fn get_c_ptr(&self) -> *const C::TfLiteInterpreterOptions {
        self.options
    }
}

impl Default for InterpreterOptions {
    fn default() -> Self {
        unsafe {
            let options = C::TfLiteInterpreterOptionsCreate();
            assert!(!options.is_null());
            Self { options }
        }
    }
}

impl Drop for InterpreterOptions {
    fn drop(&mut self) {
        unsafe { C::TfLiteInterpreterOptionsDelete(self.options) }
    }
}
