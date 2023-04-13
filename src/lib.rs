#![allow(dead_code)]
use pyo3::{prelude::*};

pub mod subclass;


pub trait MakeSubClass: MakeSubClassClone + std::fmt::Debug {
    fn add_subclass(&self, base: BaseClass) -> PyObject;
}

pub trait MakeSubClassClone {
    fn clone_box(&self) -> Box<dyn MakeSubClass>;
}

impl<T> MakeSubClassClone for T
where
    T: MakeSubClass + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn MakeSubClass> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MakeSubClass> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}


#[pyclass(subclass, unsendable)]
pub struct BaseClass {
    kind_dyn: Box<dyn MakeSubClass>,
}

#[pymethods]
impl BaseClass {
    #[staticmethod]
    fn new() -> BaseClass {
        BaseClass { 
            kind_dyn: Box::new(BaseClassBuilder{}) as Box<dyn MakeSubClass>
        }
    }

    fn clone_instance(&self) -> PyObject {
        let builder_dyn = self.kind_dyn.clone();
        let base = BaseClass {
            kind_dyn: builder_dyn.clone(),
        };

        builder_dyn.add_subclass(base)
    }
}

#[derive(Clone, Debug)]
struct BaseClassBuilder;


impl MakeSubClass for BaseClassBuilder {
    fn add_subclass(&self, base: BaseClass) -> PyObject {
        eprintln!("adding subclass for Base");
        Python::with_gil(|py| {
            Py::new(py, base).unwrap().to_object(py)
        })
    }
}





#[pyfunction]
fn make_base() -> PyResult<Py<BaseClass>> {
    Python::with_gil(|py| {
        Py::new(py, PyClassInitializer::from(BaseClass{
            kind_dyn: Box::new(BaseClassBuilder{}) as Box<dyn MakeSubClass>,
        }))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_subclass(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_base, m)?)?;
    m.add_function(wrap_pyfunction!(subclass::make_sub, m)?)?;
    m.add_class::<BaseClass>()?;
    m.add_class::<subclass::SubClass>()?;
    Ok(())
}