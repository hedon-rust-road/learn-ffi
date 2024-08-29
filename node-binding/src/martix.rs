use std::fmt::{self};

use algo::{multiply_single, Matrix};
use napi::Env;
use napi_derive::napi;

#[napi(js_name = "Matrix")]
pub struct JsMatrix {
  inner: Matrix<f64>,
}

#[napi]
impl JsMatrix {
  #[napi(constructor)]
  pub fn try_new(data: Vec<Vec<f64>>, _env: Env) -> napi::Result<Self> {
    if data.is_empty() || data[0].is_empty() {
      return Err(napi::Error::new(
        napi::Status::InvalidArg,
        "Matrix must be non-empty".to_owned(),
      ));
    }

    let row = data.len();
    let col = data[0].len();
    let data: Vec<_> = data.into_iter().flatten().collect();

    Ok(Self {
      inner: Matrix::new(data, row, col),
    })
  }

  #[napi]
  pub fn mul(&self, other: &JsMatrix) -> napi::Result<Self> {
    let result = multiply_single(&self.inner, &other.inner).unwrap();
    Ok(Self { inner: result })
  }

  #[napi]
  pub fn multiply(&self, other: Vec<Vec<f64>>, env: Env) -> napi::Result<Self> {
    let other = JsMatrix::try_new(other, env)?;
    self.mul(&other)
  }

  #[napi]
  pub fn display(&self) -> String {
    format!("{}", self.inner)
  }
}

impl fmt::Display for JsMatrix {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.inner)
  }
}
