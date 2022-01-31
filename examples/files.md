# Loading Files
A new method was added in 1.1.0, named `load_file`, this method loads code specified at path.
Give next structure:
index.neko
```
neko util = load_file("./util.neko");

log(util->join("me", "this")); @ Outputs "me this"
```
util.neko
```
public->join = dyn (x, y) {
    @ this will join x with y.
    return x + " " + y 
};
```