use pyo3::{prelude::*, types::PyDict};
use rookie::enums::Cookie;
mod browsers;
use browsers::*;

#[pyfunction]
fn version() -> PyResult<String> {
  Ok(rookie::version())
}

#[pymodule]
fn rookiepy(_py: Python, m: &PyModule) -> PyResult<()> {
  pyo3_log::init();
  m.add_function(wrap_pyfunction!(firefox, m)?)?;
  m.add_function(wrap_pyfunction!(librewolf, m)?)?;
  m.add_function(wrap_pyfunction!(chrome, m)?)?;
  m.add_function(wrap_pyfunction!(brave, m)?)?;
  m.add_function(wrap_pyfunction!(edge, m)?)?;
  m.add_function(wrap_pyfunction!(opera, m)?)?;
  m.add_function(wrap_pyfunction!(opera_gx, m)?)?;

  m.add_function(wrap_pyfunction!(chromium, m)?)?;
  m.add_function(wrap_pyfunction!(vivaldi, m)?)?;
  m.add_function(wrap_pyfunction!(arc, m)?)?;
  m.add_function(wrap_pyfunction!(chromium_based, m)?)?;
  m.add_function(wrap_pyfunction!(firefox_based, m)?)?;
  m.add_function(wrap_pyfunction!(load, m)?)?;
  m.add_function(wrap_pyfunction!(any_browser, m)?)?;

  #[cfg(target_os = "windows")]
  {
    m.add_function(wrap_pyfunction!(internet_explorer, m)?)?;
    m.add_function(wrap_pyfunction!(octo_browser, m)?)?;
  }
  #[cfg(target_os = "macos")]
  {
    m.add_function(wrap_pyfunction!(safari, m)?)?;
  }

  m.add_function(wrap_pyfunction!(version, m)?)?;
  Ok(())
}

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

    cookie_objects.push(dict.to_object(py));
  }
  Ok(cookie_objects)
}
