# Overlay: Python

## Conventions
- Type hints on all function signatures (Python 3.10+ syntax: `str | None` not `Optional[str]`)
- Use `dataclasses` or `pydantic` for structured data, not raw dicts
- Use `pathlib.Path` over `os.path` for filesystem operations
- f-strings for formatting (not `.format()` or `%`)
- Use `logging` module, not `print()` for diagnostic output

## Package Management
- Follow the project's existing tooling (pip, poetry, uv, pdm)
- Always work in a virtual environment
- Pin versions in `requirements.txt` / `pyproject.toml` for applications
- Use `pyproject.toml` as the single source of project metadata

## Error Handling
- Custom exception classes inheriting from a base project exception
- Don't catch `Exception` broadly — catch specific exceptions
- Use `contextlib.suppress(SpecificError)` instead of empty `except` blocks
- Always include context in exceptions: `raise ValueError(f"Invalid {x}: must be > 0") from e`

## Testing
- Use `pytest` (not unittest) unless the project explicitly uses unittest
- Fixtures over setup/teardown methods
- `tmp_path` fixture for filesystem tests
- `monkeypatch` for patching (not `unittest.mock.patch` decorators)
- `pytest.raises(ExceptionType, match="pattern")` for error testing

## Async Patterns
- `asyncio` for async code, `pytest-asyncio` for async tests
- Use `async with` for resource management in async contexts
- `asyncio.gather()` for concurrent operations
- `asyncio.TaskGroup` (3.11+) for structured concurrency

## Build & Verify
- `pytest` — run tests
- `mypy .` or `pyright .` — type checking
- `ruff check .` — lint (fast, replaces flake8/isort/many others)
- `ruff format .` — formatting (replaces black)

## References

- https://docs.python.org/3/
- https://peps.python.org/pep-0008/
