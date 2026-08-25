# Exit code
1

# stdout
```
  x native-cfg(native-cfg): Native Oxc CFG is available to JS plugins.
   ,-[files/index.js:1:1]
 1 | ,-> function choose(flag) {
 2 | |     if (flag) return "正常";
 3 | |     throw new Error("异常");
 4 | |   }
 5 | |   
 6 | `-> choose(true);
   `----

Found 0 warnings and 1 error.
Finished in Xms on 1 file with 1 rules using X threads.
```

# stderr
```
```
