
use std::path::PathBuf;
use rookie::{self,Cookie};

use pyo3::prelude::*;


// Wrapper struct for Cookie
#[pyclass]
pub struct PyCookie {
    pub inner: Cookie,
}


#[pymethods]
impl PyCookie {
    #[getter]
    fn domain(&self) -> &str {
        &self.inner.domain
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
    fn expires(&self) -> Option<u64> {
        self.inner.expires
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
fn libre_wolf(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::libre_wolf(domains).unwrap();
    
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
fn opera(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::opera(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
fn opera_gx(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::opera_gx(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
fn chromium(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::chromium(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
fn vivaldi(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::vivaldi(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
#[cfg(target_os = "windows")]
fn internet_explorer(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::internet_explorer(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
#[cfg(target_os = "macos")]
fn safari(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::safari(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}


#[pyfunction]
#[cfg(target_os = "windows")]
fn chromium_based(_py: Python, key_path: String, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains).unwrap();
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();
    Ok(py_cookies)
}

#[pyfunction]
#[cfg(unix)]
fn chromium_based(_py: Python, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    use rookie::BrowserConfig;

    let db_path = db_path.as_str();
    let config = BrowserConfig {
        channels: None,
        data_paths: &[db_path],
        os_crypt_name: Some("chrome"),
        osx_key_service: None,
        osx_key_user: None,
    };
    let cookies = rookie::chromium_based(&config, PathBuf::from(db_path), domains).unwrap();
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();
    Ok(py_cookies)
}


#[pyfunction]
fn firefox_based(_py: Python, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::firefox_based(PathBuf::from(db_path), domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pyfunction]
fn load(_py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyCookie>> {
    let cookies = rookie::load(domains).unwrap();
    
    let py_cookies: Vec<PyCookie> = cookies.into_iter().map(|cookie| PyCookie { inner: cookie }).collect();

    Ok(py_cookies)
}

#[pymodule]
fn rookiepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(firefox, m)?)?;
    m.add_function(wrap_pyfunction!(libre_wolf, m)?)?;
    m.add_function(wrap_pyfunction!(chrome, m)?)?;
    m.add_function(wrap_pyfunction!(brave, m)?)?;
    m.add_function(wrap_pyfunction!(edge, m)?)?;
    m.add_function(wrap_pyfunction!(opera, m)?)?;
    m.add_function(wrap_pyfunction!(opera_gx, m)?)?;
    m.add_function(wrap_pyfunction!(chromium, m)?)?;
    m.add_function(wrap_pyfunction!(vivaldi, m)?)?;
    m.add_function(wrap_pyfunction!(chromium_based, m)?)?;
    m.add_function(wrap_pyfunction!(firefox_based, m)?)?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    
    #[cfg(target_os = "windows")]
    m.add_function(wrap_pyfunction!(internet_explorer, m)?)?;

    #[cfg(target_os = "macos")]
    m.add_function(wrap_pyfunction!(safari, m)?)?;

    Ok(())
}