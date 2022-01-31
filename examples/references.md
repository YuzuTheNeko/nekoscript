# References
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
