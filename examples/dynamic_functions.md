# Dynamic Functions
Dynamic functions are basically the son of defined functions, but these do not have a name.
```
neko u = dyn () {
    log("logger!");
};

u();
```
We can call it directly though.
```
dyn () {
    log("logger!");
}();
```
Params sound like a good idea
```
dyn (x) {
    log(x);
}("yes");
```