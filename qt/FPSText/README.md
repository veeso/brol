# FPSText

Show current FPS in QML

## Implementation
  
```cpp
//Main.cpp
#include "fpstext.h"
...  
qmlRegisterType<FPSText>("FPSText", 1, 0, "FPSText");
```  

```qml
//Main.qml

//Show FPS
    FPSText {
        id: fpsTxt;
        x: 0;
        y: 0;
        z: 100;
        width: 64;
        height: 32;
        Text {
            text: "FPS " + fpsTxt.fps.toFixed(2);
        }
    }  
```
