#![allow(dead_code)]
use pyo3::prelude::*;


#[derive(Clone, Debug)]
pub enum BuilderKind {
    Base,
    Sub,
}

#[derive(Clone, Debug)]
#[pyclass(subclass)]
struct BaseClass {
    kind: BuilderKind,
}


#[pymethods]
impl BaseClass {
    fn make_another(&self) -> PyObject {
        Python::with_gil(|py| {
            match self.kind {
                BuilderKind::Base => {
                    Py::new(
                        py, 
                        PyClassInitializer::from(BaseClass { kind: BuilderKind::Base })
                    ).unwrap().to_object(py)
                },
                BuilderKind::Sub => {
                    Py::new(
                        py, 
                        PyClassInitializer::from(BaseClass { kind: BuilderKind::Sub }).add_subclass(SubClass{})
                    ).unwrap().to_object(py)
                },
            }
        })
    }
}

#[pyclass(extends=BaseClass, subclass)]
#[derive(Clone, Debug)]
struct SubClass {

}

trait SelfBuilder {
    fn build(&self) -> Self;
}

impl SelfBuilder for SubClass {
    fn build(&self) -> Self {
        SubClass {  }
    }
}

impl SelfBuilder for BaseClass {
    fn build(&self) -> Self {
        BaseClass {
            kind: BuilderKind::Base,
        }
    }
}

// trait Builder {
//     type ToBuild;
//     fn build(&self) -> Self::ToBuild;
// }

// impl Builder for BaseClass {
//     type ToBuild = BaseClass;

//     fn build(&self) -> Self::ToBuild {
//         BaseClass {
//             base: "string".to_owned(),
//         }
//     }
// }

// impl Builder for SubClass {
//     type ToBuild = SubClass;

//     fn build(&self) -> Self::ToBuild {
//         SubClass {

//         }
//     }
// }

#[pyfunction]
fn make_sub() -> PyResult<Py<SubClass>> {
    Python::with_gil(|py| {
        Py::new(py, PyClassInitializer::from(BaseClass{
            base: "One".to_owned(),
            kind: BuilderKind::Sub,
        }).add_subclass(SubClass { }))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_subclass(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_sub, m)?)?;
    m.add_class::<BaseClass>()?;
    m.add_class::<SubClass>()?;
    Ok(())
}