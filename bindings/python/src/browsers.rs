use crate::to_dict;
use pyo3::prelude::*;
use std::path::PathBuf;

/// Any browser

#[pyfunction]
pub fn any_browser(
  py: Python,
  db_path: &str,
  domains: Option<Vec<String>>,
  key_path: Option<&str>,
) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::any_browser(db_path, domains, key_path)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Common browsers

#[pyfunction]
pub fn firefox(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::firefox(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn librewolf(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::librewolf(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn chrome(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::chrome(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn arc(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::arc(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn brave(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::brave(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn edge(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::edge(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn opera(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::opera(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn opera_gx(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::opera_gx(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn chromium(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::chromium(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn vivaldi(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::vivaldi(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn firefox_based(
  py: Python,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::firefox_based(PathBuf::from(db_path), domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
pub fn load(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::load(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Windows only browsers

#[pyfunction]
#[cfg(target_os = "windows")]
pub fn octo_browser(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::octo_browser(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

#[pyfunction]
#[cfg(target_os = "windows")]
pub fn internet_explorer(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::internet_explorer(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}
#[pyfunction]
#[cfg(target_os = "windows")]
pub fn chromium_based(
  py: Python,
  key_path: String,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// MacOS browsers

#[pyfunction]
#[cfg(target_os = "macos")]
pub fn safari(py: Python, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::safari(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Unix browsers

#[pyfunction]
#[cfg(unix)]
pub fn chromium_based(
  py: Python,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<PyObject>> {
  use rookie::common::enums::BrowserConfig;

  let db_path = db_path.as_str();
  let config = BrowserConfig {
    channels: None,
    data_paths: &[db_path],
    os_crypt_name: Some("chrome"),
    osx_key_service: None,
    osx_key_user: None,
  };
  let cookies = rookie::chromium_based(&config, PathBuf::from(db_path), domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}
