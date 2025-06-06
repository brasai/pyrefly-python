use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

// Importa a struct Playground do crate pyrefly
use pyrefly::playground::Playground;
// Importa as structs de dados do playground para conversão
use pyrefly::playground::{
    Diagnostic as PyreflyDiagnostic,
    AutoCompletionItem as PyreflyAutoCompletionItem,
};

mod utils;

// Estrutura interna que encapsula a instância do Playground do Pyrefly.
// Não é exposta diretamente ao Python.
struct PyreflyAnalyzerInternal {
    playground: Playground,
}

impl PyreflyAnalyzerInternal {
    fn new() -> Self {
        // Inicializa o Playground com configurações padrão.
        // O Playground::new() já lida com a criação de um ConfigFile default
        // e um ConfigFinder constante, usando "test.py" como nome de arquivo virtual.
        PyreflyAnalyzerInternal {
            playground: Playground::new(),
        }
    }
}

// Classe Python que será exposta.
#[pyclass(name = "PyreflyAnalyzer")]
struct PyreflyAnalyzer {
    internal: PyreflyAnalyzerInternal,
}

#[pymethods]
impl PyreflyAnalyzer {
    #[new]
    fn __init__() -> PyResult<Self> {
        Ok(PyreflyAnalyzer {
            internal: PyreflyAnalyzerInternal::new(),
        })
    }

    fn update_source(&mut self, source_code: &str) -> PyResult<()> {
        self.internal.playground.update_source(source_code.to_string());
        Ok(())
    }

    fn get_diagnostics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let rust_diagnostics: Vec<PyreflyDiagnostic> = self.internal.playground.get_errors();
        let py_diagnostics = PyList::empty_bound(py);

        for diag_item in rust_diagnostics {
            let dict = PyDict::new_bound(py);
            dict.set_item("start_line", diag_item.start_line)?;
            dict.set_item("start_column", diag_item.start_col)?;
            dict.set_item("end_line", diag_item.end_line)?;
            dict.set_item("end_column", diag_item.end_col)?;
            dict.set_item("message", diag_item.message)?;
            dict.set_item("severity", utils::map_severity_to_str(diag_item.severity))?;
            dict.set_item("code", diag_item.kind)?; // `kind` já é uma String no PyreflyDiagnostic
            py_diagnostics.append(dict)?;
        }
        Ok(py_diagnostics)
    }

    fn get_autocomplete_suggestions<'py>(
        &mut self, // Playground.autocomplete precisa de &mut self
        py: Python<'py>,
        line: i32,
        column: i32,
    ) -> PyResult<Bound<'py, PyList>> {
        let rust_suggestions: Vec<PyreflyAutoCompletionItem> =
            self.internal.playground.autocomplete(line, column);
        let py_suggestions = PyList::empty_bound(py);

        for suggestion_item in rust_suggestions {
            let dict = PyDict::new_bound(py);
            dict.set_item("label", &suggestion_item.label)?;
            dict.set_item("kind", utils::map_completion_kind_to_str(suggestion_item.kind))?;
            if let Some(detail) = suggestion_item.detail {
                dict.set_item("detail", detail)?;
            }
            // O prompt especifica que insert_text é geralmente igual ao label
            dict.set_item("insert_text", &suggestion_item.label)?;
            py_suggestions.append(dict)?;
        }
        Ok(py_suggestions)
    }
}

// Define o módulo Python
#[pymodule]
fn pyrefly_native_binding(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyreflyAnalyzer>()?;
    Ok(())
}
