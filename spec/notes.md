# yallvm early notes

`dynamic` not included

Templates to stand in for generics and most uses of dynamic

most likely es6 like imports

```
import * from "awesomeFile.ya";
import {
  println,
  printf
} from std:io;

void myCoolFunction(string test) {
  println(test);
  int value = 0;
  printf("%d", value);
}

export myCoolFunction;
```

native vector/matrix support because llvm can output spirv and it'd be funny to use this as a shader lang lol
