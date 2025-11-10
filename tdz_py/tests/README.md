# Tests for Todozi

This directory contains pytest test files for the todozi package.

## Running Tests

### Run all tests
```bash
pytest
```

### Run a specific test file
```bash
pytest tests/test_agent.py
```

### Run a specific test function
```bash
pytest tests/test_agent.py::test_agent_creation
```

### Run with verbose output
```bash
pytest -v
```

### Run with coverage (requires pytest-cov)
```bash
pytest --cov=todozi --cov-report=html
```

## Test Structure

Each test file corresponds to a module in `todozi/`:
- `test_agent.py` → tests for `todozi/agent.py`
- `test_api.py` → tests for `todozi/api.py`
- etc.

## Writing Tests

Tests are written using pytest (similar to Jest for JavaScript). Example:

```python
def test_agent_creation():
    """Test Agent class creation."""
    from todozi.agent import Agent
    agent = Agent.new("Test Agent")
    assert agent.name == "Test Agent"
```

## Installation

Make sure pytest is installed:
```bash
pip install pytest
```

For coverage reports:
```bash
pip install pytest-cov
```

