use crate::to_dict;
use pyo3::prelude::*;
use std::path::PathBuf;

/// Extract Cookies from any browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (db_path, domains=None, key_path=None))]
pub fn any_browser(
  py: Python<'_>,
  db_path: &str,
  domains: Option<Vec<String>>,
  key_path: Option<&str>,
) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::any_browser(db_path, domains, key_path)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Firefox
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn firefox(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::firefox(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Zen
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn zen(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::zen(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from LibreWolf browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn librewolf(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::librewolf(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Google Chrome browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn chrome(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::chrome(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Arc browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn arc(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::arc(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Brave browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn brave(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::brave(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Microsoft Edge browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn edge(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::edge(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Opera browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn opera(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::opera(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Opera GX browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn opera_gx(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::opera_gx(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Chromium browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn chromium(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::chromium(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Vivaldi browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn vivaldi(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::vivaldi(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Firefox-based browsers
///
/// :param key_path: Path to the key file
/// :param db_path: Path to the database file
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (db_path, domains=None))]
pub fn firefox_based(
  py: Python<'_>,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::firefox_based(PathBuf::from(db_path), domains)?;
  to_dict(py, cookies)
}

/// Load Cookies from a browser
///
/// :param domains: Optional list of domains to load cookies from
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
pub fn load(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::load(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Octo browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
#[cfg(target_os = "windows")]
pub fn octo_browser(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::octo_browser(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Internet Explorer
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
#[cfg(target_os = "windows")]
pub fn internet_explorer(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::internet_explorer(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Chromium-based browsers
///
/// :param db_path: Path to the database file
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (key_path, db_path, domains=None))]
#[cfg(target_os = "windows")]
pub fn chromium_based(
  py: Python<'_>,
  key_path: String,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Safari browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (domains=None))]
#[cfg(target_os = "macos")]
pub fn safari(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<PyObject>> {
  let cookies = rookie::safari(domains)?;
  to_dict(py, cookies)
}

/// Extract Cookies from Chromium-based browsers
///
/// :param db_path: Path to the database file
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[pyo3(signature = (db_path, domains=None))]
#[cfg(unix)]
pub fn chromium_based(
  py: Python<'_>,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<PyObject>> {
  use rookie::config::Browser;

  let db_path = db_path.as_str();
  let config = Browser {
    channels: None,
    paths: vec![db_path.to_string()],
    unix_crypt_name: Some("chrome".to_string()),
    osx_key_service: None,
    osx_key_user: None,
  };
  let cookies = rookie::chromium_based(&config, PathBuf::from(db_path), domains)?;
  to_dict(py, cookies)
}
