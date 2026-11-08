use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::ffi::CString;

pub fn python_transforms(input: &[u8]) -> PyResult<Vec<(String, Vec<u8>)>> {
    let code = CString::new(include_str!("../py/extra_transforms.py")).unwrap();

    Python::attach(|py| {
        let module = PyModule::from_code(py, &code, c"extra_transforms.py", c"extra_transforms")?;
        let func = module.getattr("transform")?;
        let result = func.call1((PyBytes::new(py, input),))?;
        result.extract()
    })
}
