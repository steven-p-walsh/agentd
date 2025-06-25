use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use crate::{open, LlmInterface};
use crate::llm::LlmConfig;

/// Python wrapper around the Llm struct
#[pyclass]
pub struct PyLlm {
    inner: Box<dyn LlmInterface + Send + Sync>,
}

#[pymethods]
impl PyLlm {
    /// Generate text from a prompt
    #[pyo3(text_signature = "($self, prompt)")]
    fn generate(&self, prompt: &str) -> PyResult<String> {
        self.inner.generate(prompt)
            .map_err(|e| PyRuntimeError::new_err(format!("Generation failed: {}", e)))
    }

    /// Create a new instance with additional arguments
    #[pyo3(text_signature = "($self, args)")]
    fn with_args(_slf: PyRef<'_, Self>, _args: Vec<String>) -> PyResult<PyLlm> {
        // We need to recreate the LLM with new args
        // For now, we'll return an error and suggest using py_open_with_args
        Err(PyRuntimeError::new_err("Use py_open_with_args() instead to create an LLM with custom arguments"))
    }

    /// Get the current configuration
    #[pyo3(text_signature = "($self)")]
    fn config(&self) -> PyResult<PyLlmConfig> {
        Ok(PyLlmConfig {
            inner: self.inner.config().clone(),
        })
    }
}

/// Python wrapper around LlmConfig
#[pyclass]
pub struct PyLlmConfig {
    inner: LlmConfig,
}

#[pymethods]
impl PyLlmConfig {
    /// Get the model path
    #[getter]
    fn model_path(&self) -> String {
        self.inner.model_path.clone()
    }

    /// Get the llama executable path
    #[getter]
    fn executable_path(&self) -> String {
        self.inner.executable_path.clone()
    }

    /// Get the additional arguments
    #[getter]
    fn additional_args(&self) -> Vec<String> {
        self.inner.additional_args.clone()
    }
}

/// Open a model by name and return a PyLlm instance
/// If no model_name is provided, uses the first available model
#[pyfunction]
#[pyo3(signature = (model_name = None))]
fn py_open(model_name: Option<&str>) -> PyResult<PyLlm> {
    let actual_model_name = match model_name {
        Some(name) => name.to_string(),
        None => {
            // Get the first available model
            match crate::discover_models() {
                Ok(models) => {
                    if let Some(first_model) = models.keys().next() {
                        first_model.clone()
                    } else {
                        return Err(PyRuntimeError::new_err("No models available. Please download a model first."));
                    }
                }
                Err(e) => return Err(PyRuntimeError::new_err(format!("Failed to discover models: {}", e))),
            }
        }
    };
    
    match open(&actual_model_name) {
        Ok(llm) => Ok(PyLlm { inner: llm }),
        Err(e) => Err(PyRuntimeError::new_err(format!("Failed to open model '{}': {}", actual_model_name, e))),
    }
}

/// Open a model by name with custom arguments
/// If no model_name is provided, uses the first available model
#[pyfunction]
#[pyo3(signature = (args, model_name = None))]
fn py_open_with_args(args: Vec<String>, model_name: Option<&str>) -> PyResult<PyLlm> {
    let actual_model_name = match model_name {
        Some(name) => name.to_string(),
        None => {
            // Get the first available model
            match crate::discover_models() {
                Ok(models) => {
                    if let Some(first_model) = models.keys().next() {
                        first_model.clone()
                    } else {
                        return Err(PyRuntimeError::new_err("No models available. Please download a model first."));
                    }
                }
                Err(e) => return Err(PyRuntimeError::new_err(format!("Failed to discover models: {}", e))),
            }
        }
    };
    
    match open(&actual_model_name) {
        Ok(llm) => {
            let llm_with_args = llm.with_args(args);
            Ok(PyLlm { inner: llm_with_args })
        }
        Err(e) => Err(PyRuntimeError::new_err(format!("Failed to open model '{}': {}", actual_model_name, e))),
    }
}

/// List available models
#[pyfunction]
#[pyo3(text_signature = "()")]
fn py_list_models() -> PyResult<Vec<String>> {
    match crate::discover_models() {
        Ok(models) => Ok(models.keys().cloned().collect()),
        Err(e) => Err(PyRuntimeError::new_err(format!("Failed to discover models: {}", e))),
    }
}

/// Python module definition
#[pymodule]
pub fn agentd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_open, m)?)?;
    m.add_function(wrap_pyfunction!(py_open_with_args, m)?)?;
    m.add_function(wrap_pyfunction!(py_list_models, m)?)?;
    m.add_class::<PyLlm>()?;
    m.add_class::<PyLlmConfig>()?;
    
    // Add module-level aliases for convenience
    m.add("open", m.getattr("py_open")?)?;
    m.add("open_with_args", m.getattr("py_open_with_args")?)?;
    m.add("list_models", m.getattr("py_list_models")?)?;
    
    Ok(())
}