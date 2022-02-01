# Using Libraries
In the same folder where your main file is located, create `libraries` folder so it'd look like this:
```
- index.neko 
- libraries
    - test_lib
        - index.neko
```

In `libraries/test_lib/index.neko`, we will write the follow:
```
public = object!{
    version: "1.0.0"
};
```
Now, in `index.neko` (our main file):
```
neko lib = load_file("test_lib");

log(lib); @ Will log the object
```
Not hard right? All libraries must have an index file.

# Installing Libraries
Simply download the folder of the library and place it into your library folder.