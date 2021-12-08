``` 
    aaaa
   b    c
   b    c
    dddd
   e    f
   e    f
    gggg
```

I solved the first problem simply by checking length and maintaining a count.
The second problem provced to be more complicated, and I tried a couple approaches.
I first noted that we could derive the 6 segment numbers by comparing how 1, 4, and 7 fit inside of them.
I was able to create a solution using these attributes, however it was hard to follow, depending on state to carry through loops. (can't figure out other digits if the initial known length sets have been found)

I then realized that each digit will potentially have a unique hash that can be created by grouping how often a segment is used, and combining with which segments each number needs to activate. This proved to be the case, and allowed me to deciper each number with one pass through each line, and no dependencies on discovering a digit to deciper another.

### Total count of each segment in the 10 digits

- a = 8
- b = 6
- c = 8
- d = 7
- e = 4
- f = 9
- g = 7

### Unique hashes based on how many times a given segment is used (from above)

- 0 868497 467889
- 1 89
- 2 88747 47788
- 3 88797 77889
- 4 6789
- 5 86797 67789
- 6 867497 467789
- 7 889
- 8 8687497 4677889
- 9 868797 677889

### Initial approach, derive numbers based on how other segments fit inside

- 0 7 && !4 && 1 , len 6
- 1 len 2
- 2 LAST!
- 3 1 && 7 && !4, len 5
- 4 len 4
- 5 has the shared char from 1 & 6, len 5
- 6 !7 && !4 && !1 , len 6
- 7 len 3
- 8 len 7
- 9 4 && 7 && 1 , len 6
