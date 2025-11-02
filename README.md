Explanation and tutorial:

https://www.badprog.com/rust-python-binding-setting-up-pyo3-crate-with-maturin


# Summary
To generate the Rust binding library we're going to use Maturin.  
It's a tool used to manage the compilation and the path where the dynamic library will be generated.

Before going further, let's create a virtual environment for Python:

```bash
python -m venv .venv1
```

Then, let's activate it:

```bash
. .venv1/bin/activate
```

With our Python setup ready, let's install the Maturin Python tool:

```bash
pip install maturin
```

Use it to generate the dynamic library then allowing Python to access it:

```bash
maturin develop -m classic_operations/Cargo.toml
```

Note here that we need to explicitely specify where is the Cargo.toml file to load because we use subprojects (potentially many Cargo.toml files). 

It's also possible to go to the subproject directly and type the command without specifying the path of Cargo.toml (just "maturin develop").

It's now time to check with our Python file if our library really works.

```bash
python binding.py
```
You should see this output:

```bash
Total add: 213.0  
Total sub: -167.0  
Total div: 0.45  
Total mul: 1320.0
```