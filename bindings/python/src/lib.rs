
use std::path::PathBuf;
use rookie::{self,Cookie};

use pyo3::{prelude::*, types::PyDict};



fn to_dict(py: Python, cookies: Vec<Cookie>) -> PyResult<Vec<PyObject>> {
    
    let mut cookie_objects: Vec<PyObject> = vec![];
    for cookie in cookies {
        let dict = PyDict::new(py);
        dict.set_item("domain", cookie.domain)?;
        dict.set_item("path", cookie.path)?;
        dict.set_item("secure", cookie.secure)?;
        dict.set_item("http_only", cookie.http_only)?;
        dict.set_item("same_site", cookie.same_site)?;
        dict.set_item("expires", cookie.expires)?;
        dict.set_item("name", cookie.name)?;
        dict.set_item("value", cookie.value)?;

        // Add fields to cookie_dict using set_item
        // cookie_dict.set_item(py, "field_name", field_value);
        // Repeat for each field in the Cookie struct
        // Finally, return the cookie_dict as a PyObject
        cookie_objects.push(dict.to_object(py));
    }
    Ok(cookie_objects)
}

#[pyfunction]
fn firefox(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::firefox(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn libre_wolf(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::libre_wolf(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn chrome(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::chrome(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}


#[pyfunction]
fn brave(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::brave(domains).unwrap();
    
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn edge(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::edge(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn opera(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::opera(domains).unwrap();
    
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn opera_gx(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::opera_gx(domains).unwrap();
    
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn chromium(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::chromium(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn vivaldi(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::vivaldi(domains).unwrap();
    
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
#[cfg(target_os = "windows")]
fn internet_explorer(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::internet_explorer(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
#[cfg(target_os = "macos")]
fn safari(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::safari(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}


#[pyfunction]
#[cfg(target_os = "windows")]
fn chromium_based(py: Python, key_path: String, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
#[cfg(unix)]
fn chromium_based(py: Python, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
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
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}


#[pyfunction]
fn firefox_based(py: Python, db_path: String, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::firefox_based(PathBuf::from(db_path), domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
}

#[pyfunction]
fn load(py: Python, domains: Option<Vec<&str>>) -> PyResult<Vec<PyObject>> {
    let cookies = rookie::load(domains).unwrap();
    let cookies = to_dict(py, cookies)?;

    Ok(cookies)
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