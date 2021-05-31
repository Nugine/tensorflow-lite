use std::ptr;

use crate::bindings as C;
use crate::error::Result;
use crate::model::Model;
use crate::tensor::Tensor;

mod options;

pub use self::options::InterpreterOptions;

pub struct Interpreter {
    interpreter: *mut C::TfLiteInterpreter,
}

unsafe impl Send for Interpreter {}
unsafe impl Sync for Interpreter {}

impl Interpreter {
    fn create(model: &Model, options: Option<&InterpreterOptions>) -> Result<Self> {
        let model = model.get_c_ptr();
        let options = options.map(|opt| opt.get_c_ptr()).unwrap_or(ptr::null());
        unsafe {
            let interpreter = C::TfLiteInterpreterCreate(model, options);
            if interpreter.is_null() {
                bail!("failed to create interpreter")
            }
            Ok(Self { interpreter })
        }
    }

    pub fn new(model: &Model) -> Result<Self> {
        Self::create(model, None)
    }

    pub fn new_with_options(model: &Model, options: &InterpreterOptions) -> Result<Self> {
        Self::create(model, Some(options))
    }

    pub fn default_options() -> InterpreterOptions {
        InterpreterOptions::default()
    }

    pub fn allocate_tensors(&mut self) -> Result<()> {
        unsafe {
            bail!(@check C::TfLiteInterpreterAllocateTensors(self.interpreter));
            Ok(())
        }
    }

    pub fn get_input_tensor_count(&self) -> u32 {
        unsafe {
            let count = C::TfLiteInterpreterGetInputTensorCount(self.interpreter);
            assert!(count >= 0);
            count as u32
        }
    }

    pub fn get_input_tensor_mut(&mut self, index: u32) -> Option<&mut Tensor> {
        if index >= self.get_input_tensor_count() {
            return None;
        }
        unsafe {
            let tensor = C::TfLiteInterpreterGetInputTensor(self.interpreter, index as i32);
            Some(Tensor::from_c_mut_ptr(tensor))
        }
    }

    pub fn get_output_tensor_count(&self) -> u32 {
        unsafe {
            let count = C::TfLiteInterpreterGetOutputTensorCount(self.interpreter);
            assert!(count >= 0);
            count as u32
        }
    }

    pub fn get_output_tensor(&self, index: u32) -> Option<&Tensor> {
        if index >= self.get_output_tensor_count() {
            return None;
        }
        unsafe {
            let tensor = C::TfLiteInterpreterGetOutputTensor(self.interpreter, index as i32);
            Some(Tensor::from_c_ptr(tensor))
        }
    }

    // FIXME: is it safe?
    pub fn invoke(&mut self) -> Result<()> {
        unsafe { bail!(@check C::TfLiteInterpreterInvoke(self.interpreter)) };
        Ok(())
    }
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        unsafe { C::TfLiteInterpreterDelete(self.interpreter) }
    }
}
