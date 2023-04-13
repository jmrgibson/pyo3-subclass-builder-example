use pyo3::prelude::*;

use crate::{BaseClass, MakeSubClass};

#[pyclass(extends=BaseClass, subclass)]
pub struct SubClass {}

#[derive(Clone, Debug)]
pub struct SubClassBuilder;

impl MakeSubClass for SubClassBuilder {
    fn add_subclass(&self, base: BaseClass) -> PyObject {
        eprintln!("adding subclass for SubClass");
        let sub = SubClass{};
        let instance = PyClassInitializer::from(base).add_subclass(sub);
        Python::with_gil(|py| {
            Py::new(py, instance).unwrap().to_object(py)
        })
    }
}

#[pyfunction]
pub fn make_sub() -> PyResult<Py<SubClass>> {
    Python::with_gil(|py| {
        Py::new(py, PyClassInitializer::from(BaseClass{
            kind_dyn: Box::new(SubClassBuilder{}) as Box<dyn MakeSubClass>,
        }).add_subclass(SubClass { }))
    })
}