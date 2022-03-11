# SI - Rozpoznawanie Numerów

![Preview](assets/image.png)

## Opis

Aplikacja, którą można nauczyć rozpoznawać odręcznie pisane cyfry od 0-9.

Gotową aplikację można pobrać [tutaj](https://github.com/rosowskimik/ai-numery/releases).

## Budowa z kodu źródłowego

### *Linux
W wypadku linuxa trzeba zainstalować potrzebne biblioteki, np:

* Ubuntu:
  `apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`
* Fedora:
  `dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel`
* Arch:
  `pacman -S libspeechd libxkbcommon-x11 openssl`

Na pozostałych systemach aplikacja powinna dać się zbudować od ręki.

```sh
git clone https://github.com/rosowskimik/ai-numery.git
cd ai-numery
cargo build --release
```
