use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;

fn get_executable_dir() -> Option<PathBuf> {
    match std::env::current_exe() {
        Ok(mut path) => {
            path.pop();
            Some(path)
        }
        Err(_) => None,
    }
}

fn get_stdlib_dir() -> Option<PathBuf> {
    let exec_dir = get_executable_dir()?;
    let mut path = exec_dir.clone();
    path.push("stdlib");
    Some(path)
}

fn get_site_dir() -> Option<PathBuf> {
    let exec_dir = get_executable_dir()?;
    let mut path = exec_dir.clone();
    path.push("site-packages");
    Some(path)
}

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let site_dir = get_site_dir().expect("Failed to get site directory");
        let stdlib_dir = get_stdlib_dir().expect("Failed to get stdlib directory");
        let mut lib_dynload_dir = stdlib_dir.clone();
        lib_dynload_dir.push("lib-dynload");


        let sys_module = PyModule::import(py, "sys")?;
        let new_list = PyList::empty(py);
        sys_module.setattr("path", new_list)?;
        let sys_path: &PyList = pyo3::PyTryInto::try_into(sys_module.getattr("path")?)?;
        sys_path.append(stdlib_dir.to_str().unwrap())?;
        sys_path.append(lib_dynload_dir.to_str().unwrap())?;

        let python_code = format!("import site\nsite.addsitedir('{}')\n", site_dir.to_str().unwrap());
        py.run(&python_code, None, None)?;
        py.run("import angrmanagement.__main__\nangrmanagement.__main__.main()\n", None, None)?;

        Ok(())
    })
}
