use std::path::Path;

use crate::bindings as C;
use crate::error::Result;
use crate::utils;

pub struct Model {
    model: *mut C::TfLiteModel,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    pub fn create_from_file(model_path: &Path) -> Result<Model> {
        let path = utils::path_to_cstring(model_path)?;
        unsafe {
            let model = C::TfLiteModelCreateFromFile(path.as_ptr());
            if model.is_null() {
                bail!("failed to create model from file")
            }
            Ok(Self { model })
        }
    }

    pub(crate) fn get_c_ptr(&self) -> *const C::TfLiteModel {
        self.model
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { C::TfLiteModelDelete(self.model) }
    }
}
