# NekoScript
That's it. I just made this programming language to keep myself busy, and because of my passion for nekos, I went with that name.

You are more than welcome to report bugs, note that some stuff is not yet implemented.

Here is a small example on how to define variables:
```
neko uwu = 1;

log(uwu);
```
This language relies on semicolons quite a lot, remember that.

There are a small variety of types:
```
log(typeof([])); @ array
log(typeof(object!{})); @ object
log(typeof(dyn () {})); @ dyn 
log(typeof("hello")); @ text 
log(typeof(1)); @ int
log(typeof(true)); @ boolean
log(typeof(null)); @ this will not work, but it's type is null 
```
Accessing object properties is not really hard, it kind of is a mix-up between C++ and JavaScript:
```
neko uwu = object!{
    lol: true,
    yes: 1
};

log(uwu->lol);

uwu->yes = 10;

log(uwu->yes);
```
Arrays are simple, they look pretty much like JavaScript.
```
neko arr = [
    "uwu",
    object!{},
    0
];

log(arr->at(0)); @ Will print uwu
log(arr); @ Gibberish
```
There is only one kind of loop implemented, that is `while` loop:
```
neko x = 0;

while x != 5 {
    x += 1;
};

log(x);
```
Many operators are not implemented, so yeah.

Ternary operators are lovely, it would hurt me not to add it:
```
neko u = 10;

@ Should output `yes`, might break? I do not think so.
log(u == 10 ? "yes" : "no");
```
Dynamic functions, they are basically the son of defined functions, but these do not have a name.
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

The return keyword does exist, we can use it:
```
define uwu() {
    return "bozo";
};

log(uwu());
```

I forgot that `system_time()` exists, it just returns current milliseconds since the epoch.

If statements exist by the way
```
if (1 == 1) {
    log("is 1");
} else {
    log("is not 1");
};
```

Might have forgotten something, but that is pretty much all for now.

# Things to keep in mind
NekoScript uses references behind what you see, this means, the current code:
```
neko x = 1;
neko a = x;
```
Both variables hold and point to the same value allocated in heap, therefore modifying one value will also modify the other.
```
neko x = 1;
neko a = x;

a = 10; @ You might expect x to still be 1

log(x, a); @ Will print 10 10
```
Might sound like a bad idea, however it is a great thing to keep in mind since the data will not be cloned every time you use the variable.

Native data types do have native methods, for example int conversion:
```
neko s = 1;
log(s->to_string()) @ You won't notice any difference, but if you use typeof you'll see magic.
```
Note that not all data types have methods yet, this can be found [here](https://github.com/Rubenennj/nekoscript/tree/dev/src/native/prototypes)

# Loading other files
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
Note though, that due to lack of cache this will be loaded every time it is used.