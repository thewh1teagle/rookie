use crate::to_dict;
use pyo3::prelude::*;
use std::path::PathBuf;

/// Extract Cookies from any browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn any_browser(
  py: Python<'_>,
  db_path: &str,
  domains: Option<Vec<String>>,
  key_path: Option<&str>,
) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::any_browser(db_path, domains, key_path)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Firefox
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn firefox(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::firefox(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Zen
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn zen(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::zen(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from LibreWolf browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn librewolf(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::librewolf(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Google Chrome browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn chrome(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::chrome(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Arc browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn arc(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::arc(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Brave browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn brave(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::brave(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Microsoft Edge browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn edge(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::edge(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Opera browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn opera(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::opera(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Opera GX browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn opera_gx(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::opera_gx(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Chromium browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn chromium(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::chromium(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Vivaldi browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn vivaldi(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::vivaldi(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Firefox-based browsers
///
/// :param key_path: Path to the key file
/// :param db_path: Path to the database file
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn firefox_based(
  py: Python<'_>,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::firefox_based(PathBuf::from(db_path), domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Load Cookies from a browser
///
/// :param domains: Optional list of domains to load cookies from
/// :return: A list of dictionaries of cookies
#[pyfunction]
pub fn load(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::load(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Octo browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[cfg(target_os = "windows")]
pub fn octo_browser(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::octo_browser(domains)?;

  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Internet Explorer
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[cfg(target_os = "windows")]
pub fn internet_explorer(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::internet_explorer(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Chromium-based browsers
///
/// :param db_path: Path to the database file
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[cfg(target_os = "windows")]
pub fn chromium_based(
  py: Python<'_>,
  key_path: String,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Safari browser
///
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[cfg(target_os = "macos")]
pub fn safari(py: Python<'_>, domains: Option<Vec<String>>) -> PyResult<Vec<Py<PyAny>>> {
  let cookies = rookie::safari(domains)?;
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}

/// Extract Cookies from Chromium-based browsers
///
/// :param db_path: Path to the database file
/// :param domains: Optional list of domains to extract only from them
/// :return: A list of dictionaries of cookies
#[pyfunction]
#[cfg(unix)]
pub fn chromium_based(
  py: Python<'_>,
  db_path: String,
  domains: Option<Vec<String>>,
) -> PyResult<Vec<Py<PyAny>>> {
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
  let cookies = to_dict(py, cookies)?;

  Ok(cookies)
}
