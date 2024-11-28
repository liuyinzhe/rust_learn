use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sum_as_int(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}


/// A Python module implemented in Rust.
#[pymodule]
fn maturin_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_int, m)?)?;
    Ok(())
}
