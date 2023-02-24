# marui - Find circular imports in Python projects.

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

In the current version (0.1.0), `marui` can only find direct circular imports of Python modules. The plan for the next release is to extend this to finding circular import chains of any length using [directed graphs](https://en.wikipedia.org/wiki/Directed_graph) and  [strongly connected components](https://en.wikipedia.org/wiki/Strongly_connected_component).

# Installation
At the moment, the best (and only way) to install marui is to clone this repo and to install with `cargo`:
```shell
$ gh repo clone jan-krecke/marui
$ cd marui
$ cargo install --path .
```

# Feeback and contribution
If you want to use this tool and find any problems, feel free to open a PR or an issue :-).