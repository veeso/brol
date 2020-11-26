# Brol

*Brol ~ en Français de Belgique: Désordre, bazar.*

Developed by Christian Visintin

- [Brol](#brol)
  - [Projects](#projects)
    - [C](#c)
    - [CPP](#cpp)
    - [Java](#java)
    - [JavaScript](#javascript)
    - [PHP](#php)
    - [Python](#python)
    - [Qt](#qt)
    - [Rust](#rust)
  - [Code of conduct](#code-of-conduct)
  - [License](#license)

## Projects

### C

- **[easyfb](c/easyfb)**: simple library to interact with the framebuffer (read/write you know). **MIT license**
- **[elapsed-ms](c/elapsed-us)**: How to get elapsed time in microseconds
- **[jannson](c/jannson)**: an example which shows how to use jannson JSON library
- **[getline](c/getline)**: I reimplemented getline in case it's necessary
- **[progress_bar](c/progress_bar)**: A simple progres bar in C
- **[settimeofday](c/settimeofday)**: An example program which shows how to set system time
- **[sigint](c/sigint)**: A simple SIGINT handler
- **[traslate-bit-size](c/traslate-bit-size)**: Traslate a numeric value into another bit power (e.g. 31,5 => 255,8)
- **[TunTap](c/TunTap)**: create and perform I/O on TUN/TAPs
- **[uintToAscii](c/uintToAscii)**: a converter for uintx_t to ASCII buffer
- **[union](c/union)**: An example which shows how to use Union with data buffers and structures (verify useful for example for protocols).

---

### CPP

- **[aixlog](cpp/aixlog/main.cpp)**: Aixlog example implementation
- **[base64pp](cpp/base64pp)**: Simple base64 library in C++
- **[cli11](cpp/cli11)**: Simple CLI11 example
- **[fstream-doesnot-throw](cpp/fstream-doesnot-throw)**: nothing, just a proof that fstream doesn't throw if was not possible to open it
- **[ftpcurlpp](cpp/ftpcurlpp)**: Ever wondered how to use curlpp with FTP? I did and I couldn't find any example on how to use it...
- **[getdir](cpp/getdir)**: Get all files in a directry
- **[libzip](cpp/libzip)**: An example which shows how to use libzip
- **[logrotator](cpp/logrotator)**: A simple log rotator library in c++
- **[lunasvg](cpp/lunasvg)**: Convert SVG to PNG using [liblunasvg](https://github.com/sammycage/lunasvg) and libpng 
- **[lz4](cpp/lz4)**: an example on how to use lz4 in c++
- **[mosquittopp](cpp/mosquittopp)**: an example on how to use mosquittopp in c++
- **[nlohmann-query](cpp/nlohmann-query)**: An algorithm to make recursive query in nlohmann (e.g. find a.b[1].c in a JSON)
- **[plog](cpp/plog)**: Plog example implementation
- **[tzoffset](cpp/tzoffset)**: Get current timezone offset in C++
- **[utils](cpp/utils)**: utilities (for filesystem, strings, those stuff you know)
- **[xml2](cpp/xml2)**: libxml2 example

---

### Java

- **[ISO3166](java/ISO3166)**: ISO3166 utils
- **[ISO8601](java/ISO8601)**: ISO8601 utils
- **[MySqlDateTime](java/MySqlDateTime)**: MySQL/MariaDB date parser

---

### JavaScript

- **[range](js/range)**: A function which creates array of numbers from ranges of numbers
- **[strptime](js/strptime)**: exactly strptime

---

### PHP

- **[json-node](php/json-node)**: find a JSON leaf through query
- **[requests](php/requests)**: Send GET/POST requests using PHP with cURL
- **[resize-image](php/resize-image)**: Resize an image using Imagick

---

### Python

- **[base64-cli](python/base64)**: Base64 CLI encoder/decoder written in Python3
- **[crypter](python/crypter)**: AES CLI encrypter/decrypter written in Python3
- **[logrotate-cli](python/logrotate-cli)**: CLI application which can be used to rotate logs. Is not a daemon, must be called manually. *I Should really make a Pypi binary out of this*
- **[modem-utils](python/modem-utils)**: Utility to query modem
- **[json-patch](python/json-patch)**: Utility to apply missing keys from one JSON into another
- **[netaddr-calc](python/netaddr-calc)**: Utility to calculate network addresses and stuff like that
- **[struncate](python/struncate)**: I swear nobody has ever did this before. A damn function to truncate the first n bytes from a file **without** creating a new one.
- **[subvar](python/subvar)**: Replace `$VARIABLES` and `${VARIABLES}` in files using environment.
- **[wikidata-search](python/wikidata-search)**: Simple Python3 script which searches on wikidata the provided input and if the result is a person, retrieves metadata for it

---

### Qt

- **[filehandler](qt/filehandler)**: Simple file I/O QML utility
- **[FPSText](qt/FPSText)**: Show FPS in QML
- **[Netif](qt/Netif)**: Simple network interface QML utilty
- **[Pong](qt/Pong)**: YES
- **[Process](qt/Process)** run subprocesses from QML

---

### Rust

- **[console](rust/console)**: Console utils
- **[file-utils](rust/file-utils)**: File utilities
- **[git](rust/git)**: Git utilities. **GPL3 License**
- **[pipe](rust/pipe)**: UNIX Pipes Façade for Rust. **GPL3 License**
- **[progress_bar](rust/progress_bar)**: Simple progress bar in rust.
- **[sftp-client](rust/sftp-client)**: Just a minimalist SFTP CLI client. (Don't complain about error handling)
  - Requires [chrono](https://github.com/chronotope/chrono)
  - Requires [rpassword](https://github.com/conradkleinespel/rpassword)
  - Requires [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- **[ssh-client](rust/ssh-client)**: Just a minimalist SSH client.
  - Requires [rpassword](https://github.com/conradkleinespel/rpassword)
  - Requires [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- **[subproc](rust/subproc)**: UNIX subprocess with named pipes. **GPL3 License**

---

## Code of conduct

View repository [Code of conduct](CODE_OF_CONDUCT.md)

---

## License

Licensed under WTFPL
See the entire license text [HERE](LICENSE.txt)

Some code is under different license (Don't worry, is open-source anyway). If a project has a different license is reported in the project directory.
