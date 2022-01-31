# Objects
The `object!` keyword is used to achieve this:
```
object!{
    data: true
};
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