
use std::{fmt::{self}, time::SystemTime, path::PathBuf};
use rookie::{self, enums::{CookieToString,Cookie}};
use pyo3::types::{PyFloat, PyString, PyList};
use pyo3::prelude::*;


// Wrapper struct for Cookie
#[pyclass]
pub struct PyCookie {
    pub inner: Cookie,
}



#[pymethods]
impl PyCookie {
    #[getter]
    fn host(&self) -> &str {
        &self.inner.host
    }

    #[getter]
    fn path(&self) -> &str {
        &self.inner.path
    }

    #[getter]
    fn secure(&self) -> bool {
        self.inner.secure
    }
    #[getter]
    fn expires(&self) -> PyResult<String> {
        match self.inner.expires.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Ok(duration.as_secs().to_string()),
            Err(_) => Ok("Invalid duration".to_string())
        }
    }
    

    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    #[getter]
    fn value(&self) -> &str {
        &self.inner.value
    }

    #[getter]
    fn http_only(&self) -> bool {
        self.inner.http_only
    }

    #[getter]
    fn same_site(&self) -> i64 {
        self.inner.same_site
    }
}

#[pyfunction]
fn firefox(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::firefox(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
fn chrome(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::chrome(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}


#[pyfunction]
fn brave(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::brave(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
fn edge(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::edge(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}


#[pyfunction]
fn chromium_based(_py: Python, key_path: String, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pymodule]
fn rookiepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(firefox, m)?)?;
    m.add_function(wrap_pyfunction!(chrome, m)?)?;
    m.add_function(wrap_pyfunction!(brave, m)?)?;
    m.add_function(wrap_pyfunction!(edge, m)?)?;
    m.add_function(wrap_pyfunction!(chromium_based, m)?)?;
    Ok(())
}