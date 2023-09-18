use pyo3::prelude::*;

mod run;


#[pyclass]
struct Callback {
    #[allow(dead_code)] // callback_function is called from Python
    callback_function: Box<dyn Fn(&PyAny) -> PyResult<()> + Send>,
}

#[pymethods]
impl Callback {
    fn __call__(&self, python_api: &PyAny) -> PyResult<()> {
        (self.callback_function)(python_api)
    }
}

#[pyfunction]
fn rust_register_callback(python_api: &PyAny) -> PyResult<()> {
    println!("This is rust_register_callback");
    let message: String = "a captured variable".to_string();
    Python::with_gil(|py| {
        let callback = Box::new(Callback {
            callback_function: Box::new(move |python_api| {
                rust_callback(python_api, message.clone())
            }),
        });
        python_api
            .getattr("set_callback")?
            .call1((callback.into_py(py),))?;
        Ok(())
    })
}

#[pyfunction]
fn rust_callback(python_api: &PyAny, message: String) -> PyResult<()> {
    println!("This is rust_callback");
    run::run();
    println!("Message = {}", message);
    python_api.getattr("some_operation")?.call0()?;
    Ok(())
}

#[pymodule]
#[pyo3(name = "rust_module")]
fn quantum_network_stack(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(rust_register_callback, module)?)?;
    module.add_function(wrap_pyfunction!(rust_callback, module)?)?;
    module.add_class::<Callback>()?;
    Ok(())
}
