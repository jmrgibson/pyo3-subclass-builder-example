use pyo3::{prelude::*};

pub trait MakeSubClass: MakeSubClassClone + std::fmt::Debug {
    fn new_with_base_class(&self, base: BaseClass) -> PyObject;
}

/// this just exists to clone a boxed dyn trait
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
            kind_dyn: Box::new(BaseClassBuilder{}) as Box<dyn MakeSubClass>,
        }
    }

    fn clone_instance(&self) -> PyObject {
        let builder_dyn = self.kind_dyn.clone();
        let base = BaseClass {
            kind_dyn: builder_dyn.clone(),
        };

        builder_dyn.new_with_base_class(base)
    }
}

#[derive(Clone, Debug)]
struct BaseClassBuilder;


impl MakeSubClass for BaseClassBuilder {
    fn new_with_base_class(&self, base: BaseClass) -> PyObject {
        eprintln!("adding subclass for Base");
        Python::with_gil(|py| {
            Py::new(py, base).unwrap().to_object(py)
        })
    }
}

mod subclass {
    use pyo3::prelude::*;

    use super::{BaseClass, MakeSubClass};

    #[pyclass(extends=BaseClass, subclass)]
    pub struct SubClass {}

    #[derive(Clone, Debug)]
    pub struct SubClassBuilder;

    impl MakeSubClass for SubClassBuilder {
        fn new_with_base_class(&self, base: BaseClass) -> PyObject {
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