# Modules
Modules act just like files, but they can be used to load libraries or packages you installed on your own.

There are some native modules that can be found [here](https://github.com/Rubenennj/nekoscript/tree/dev/src/native/modules)

```
neko utils = load_file("util");

@ Will print "this text"
log(
    utils->format("this", "text");
);
```