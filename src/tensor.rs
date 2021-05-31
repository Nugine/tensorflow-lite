use std::mem;
use std::ptr::NonNull;
use std::slice;

use crate::bindings as C;

mod private {
    pub trait Sealed {}
}

pub trait TensorType: private::Sealed {
    const TYPE_ID: C::TfLiteType::Type;
}

macro_rules! bind_tensor_type {
    ($ty:ty, $id: ident) => {
        impl private::Sealed for $ty {}
        impl TensorType for $ty {
            const TYPE_ID: C::TfLiteType::Type = C::TfLiteType::$id;
        }
    };
}

bind_tensor_type!(u8, kTfLiteUInt8);
// bind_tensor_type!(u16, kTfLiteUInt16); // FIXME: where is `kTfLiteUInt16`?
bind_tensor_type!(u32, kTfLiteUInt32);
bind_tensor_type!(u64, kTfLiteUInt64);

bind_tensor_type!(i8, kTfLiteInt8);
bind_tensor_type!(i16, kTfLiteInt16);
bind_tensor_type!(i32, kTfLiteInt32);
bind_tensor_type!(i64, kTfLiteInt64);

// bind_tensor_type!(C::TfLiteFloat16, kTfLiteFloat16); // TODO: provide f16?
bind_tensor_type!(f32, kTfLiteFloat32);
bind_tensor_type!(f64, kTfLiteFloat64);

#[repr(transparent)]
pub struct Tensor(C::TfLiteTensor);

unsafe impl Send for Tensor {}
unsafe impl Sync for Tensor {}

impl Tensor {
    pub(crate) unsafe fn from_c_ptr<'a>(tensor: *const C::TfLiteTensor) -> &'a Self {
        &*tensor.cast()
    }

    pub(crate) unsafe fn from_c_mut_ptr<'a>(tensor: *mut C::TfLiteTensor) -> &'a mut Self {
        &mut *tensor.cast()
    }

    pub(crate) fn as_c_ptr(&self) -> *const C::TfLiteTensor {
        unsafe { mem::transmute(self) }
    }

    // pub(crate) fn as_c_mut_ptr(&mut self) -> *mut C::TfLiteTensor {
    //     unsafe { mem::transmute(self) }
    // }

    pub fn byte_size(&self) -> usize {
        unsafe { C::TfLiteTensorByteSize(self.as_c_ptr()) }
    }

    // pub unsafe fn copy_from_buffer(&mut self, buf: &[u8]) -> Result<()> {
    //     let len = buf.len();
    //     assert_eq!(len, self.byte_size());

    //     let ptr = buf.as_ptr().cast::<c_void>();
    //     bail!(@check C::TfLiteTensorCopyFromBuffer(self.as_c_mut_ptr(), ptr, len));
    //     Ok(())
    // }

    fn get_typed_ptr<T: TensorType>(&self) -> Option<NonNull<T>> {
        let tensor = self.as_c_ptr();
        unsafe {
            let ty = C::TfLiteTensorType(tensor);
            if ty != T::TYPE_ID {
                return None;
            }
            let data = C::TfLiteTensorData(tensor);
            if data.is_null() {
                return None;
            }
            Some(NonNull::new_unchecked(data.cast::<T>()))
        }
    }

    pub fn get_typed_slice<T: TensorType>(&self) -> Option<&[T]> {
        let ptr = self.get_typed_ptr::<T>()?;
        let len = self.byte_size() / mem::size_of::<T>();
        unsafe { Some(slice::from_raw_parts(ptr.as_ptr(), len)) }
    }

    pub fn get_typed_mut_slice<T: TensorType>(&mut self) -> Option<&mut [T]> {
        let ptr = self.get_typed_ptr::<T>()?;
        let len = self.byte_size() / mem::size_of::<T>();
        unsafe { Some(slice::from_raw_parts_mut(ptr.as_ptr(), len)) }
    }
}
