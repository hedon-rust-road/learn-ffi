# learn-ffi

- napi
- pyo3

## rye

```bash
rye init python-binding --build-system maturin
rye sync
rye install maturin
rye add --dev pip
rye add --dev ipython

maturin develop
rye run ipython
```

## pyo3

Key points:

- If you change the project name:
  - `pyproject.toml` indicates the project `name`, `module-name`
  - `Directory name`
- The import name in `__init__.py` is consistent
- Remember to introduce the newly written function/class under `lib.rs` and in `__init__.py`
- Use `magic function` to make code more python
