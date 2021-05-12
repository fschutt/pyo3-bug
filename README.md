# pyo3-bug

Bug example for PyO3 bug:

```python
from azul import *

wrapper = Wrapper()
child = Child()

print(child.flag)               # ok: prints True (default)
print(wrapper.child.flag)       # ok: prints True (default)

# set both to False
child.flag = False
wrapper.child.flag = False

print(child.flag)               # ok: prints False (changed)
print(wrapper.child.flag)       # error: still prints True (not changed)
```