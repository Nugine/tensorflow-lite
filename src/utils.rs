use std::ffi::CString;
use std::path::Path;

use crate::error::Result;

#[macro_export]
macro_rules! bail {
    ($msg: literal) => {
        return Err($crate::error::Error::from_msg($msg))
    };
    (@check $status: expr) => {{
        let status = $status;
        if status != $crate::bindings::TfLiteStatus::kTfLiteOk {
            return Err($crate::error::Error::from_status($status));
        }
    }};
}

pub fn path_to_cstring(path: &Path) -> Result<CString> {
    let bytes = match path.to_str() {
        Some(s) => s.as_bytes(),
        None => bail!("expected utf-8 path"),
    };

    if bytes.contains(&b'\0') {
        bail!("the str contains NUL");
    }

    unsafe { Ok(CString::from_vec_unchecked(bytes.to_vec())) }
}
