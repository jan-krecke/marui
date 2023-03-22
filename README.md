# marui - Find circular imports in Python projects.
[![Crate](https://img.shields.io/crates/v/marui.svg)](https://crates.io/crates/marui)

At work I work with a relatively large Python code base. Sometimes I find myself accidentally adding circular dependencies between modules. This leads to the classical

```shell
ImportError: cannot import name 'A' from partially initialized module 'B'
```

marui mitigates this problem by finding circular imports before you run your CI suite.

# Usage
In a Python project (characterized by a `pyproject.toml` being present), simply run
```shell
$ marui .
```

# Limitations

In its current version (`0.2.0`), `marui` can only find imports from within project that are imported with their fully qualified path. For example, if you have a project structure like this:
```
.
├── pyproject.toml
├── my_package 
│   ├── a
│   │   ├── __init__.py
│   │   └── b.py
│   └── c
│       ├── __init__.py
│       └── d.py
└── ...
```
`b.py` can import `d.py` as
```python
import my_package.c.d 
```
but as not as
```python
import c.d
```
This will be addressed in the next version.

# Installation
If you have cloned this repository, build and install marui with `cargo`:
```
$ gh repo clone jan-krecke/marui
$ cd marui
$ cargo install --path .
```

Alternatively, just get `marui` directly from `crates.io`:

```
$ cargo install marui
```

# Feeback and contribution
If you want to use this tool and find any problems, feel free to open a PR or an issue :-).