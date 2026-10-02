# Backend build validation

Дата: 2026-10-02. Проверка проведена в WSL distribution `Ubuntu`, исходники: `C:/Users/sumin/Documents/ChatGPT/flussonic/work/nvr-fork`. Репозиторий не менялся для подготовки зависимостей; ошибки кода и теста не исправлялись.

## Окружение

В Ubuntu установлены GCC 13.3.0, CMake 3.28.3 и backend development packages (SQLite, cJSON, curl, zlib, Mosquitto, YAML, libuv, OpenSSL и др.). `dpkg --audit` и `apt-get check` завершились без ошибок.

В `/opt/lightnvr-test-deps` установлены локальные shared dependencies:

- FFmpeg 7.1, собран из официального `https://ffmpeg.org/releases/ffmpeg-7.1.tar.xz` с shared libraries; static libs, программные утилиты, docs и x86 assembly отключены. Codec/demuxer/muxer features не отключались.
- Mbed TLS 3.6.7, собран из официального release `https://github.com/Mbed-TLS/mbedtls/releases/tag/mbedtls-3.6.7`; официальный `.tar.bz2` SHA-256 sidecar проверен. Shared libs включены, programs и tests отключены. Это соответствует API, которого ожидает проект; системный Ubuntu mbedTLS 2.28 вызывал compile error из-за `mbedtls_sha256` типа `void`.
- llhttp 9.2.1, release tag `release/v9.2.1`, клонирован/собран вне репозитория.

Для configure, link и запуска бинарей нужны переменные:

```bash
export PKG_CONFIG_PATH=/opt/lightnvr-test-deps/lib/pkgconfig
export LD_LIBRARY_PATH=/opt/lightnvr-test-deps/lib
export LIBRARY_PATH=/opt/lightnvr-test-deps/lib
export CMAKE_PREFIX_PATH=/opt/lightnvr-test-deps
```

`LIBRARY_PATH` требуется linker для поиска llhttp в нестандартном prefix. CMake configure обнаружил FFmpeg versions avcodec 61.19.100, avformat 61.7.100, avutil 59.39.100, swscale 8.3.100, swresample 5.3.100; mbedtls/mbedcrypto/mbedx509 3.6.7; llhttp 9.2.1; SQLite, curl, zlib, libyaml, Mosquitto и libuv.

## Configure и build

Build tree, сохранённый в WSL Ubuntu: `/root/nvr-backend-verification-mbedtls367`.

```bash
cmake -S /mnt/c/Users/sumin/Documents/ChatGPT/flussonic/work/nvr-fork \
  -B /root/nvr-backend-verification-mbedtls367 \
  -DCMAKE_PREFIX_PATH=/opt/lightnvr-test-deps \
  -DENABLE_LITERT=OFF -DENABLE_SOD=OFF -DBUILD_TESTS=ON
cmake --build /root/nvr-backend-verification-mbedtls367 --parallel 2
```

Обе команды завершились с кодом 0. Включены full build profile, go2rtc integration, MQTT, YAML и unit-test targets. LiteRT и SOD выключены. CMake custom target сгенерировал `web/js/version.js`; tracked file остался без изменений.

## Unit tests

Полный набор CTest запущен так:

```bash
ctest --test-dir /root/nvr-backend-verification-mbedtls367 \
  --output-on-failure --parallel 2
```

Результат: **144/145 passed, 1 failed**. Единственный сбой — `test_url_utils`, `test_url_apply_credentials_replaces_existing_credentials` в `tests/unit/test_url_utils.c:22`: ожидание содержит `%3A`, функция возвращает `%3a` для percent-encoded двоеточия. Остальные 144 теста прошли.

Тест новых UI preferences/favorites запускался отдельно после сборки актуального test source:

```bash
ctest --test-dir /root/nvr-backend-verification-mbedtls367 \
  --output-on-failure -R ui_preferences
```

Результат: `test_api_handlers_ui_preferences` — **1/1 passed**.

Параллельные изменения source tree после полной сборки не были применены мной. Перед повторной проверкой текущего дерева после новых source edits запустить incremental `cmake --build` и соответствующие CTest targets в том же build tree.

## Проверка `test_url_utils` на baseline HEAD

Повторная проверка выполнена на commit `03767df2d6cbebfa388be123fa5d305a33698674`. Unit test был собран в указанном CMake build tree из исходника и теста, совпадающих с этим HEAD; отличие рабочей копии и индекса отсутствует (`git diff --exit-code HEAD -- src/core/url_utils.c include/core/url_utils.h tests/unit/test_url_utils.c` и `git diff --cached --exit-code HEAD -- ...` вернули 0). Blob IDs в HEAD и рабочей копии совпадают:

- `src/core/url_utils.c`: `e864ca8362f188cd87ee14ebf6d3b9ad1f71cafe`
- `include/core/url_utils.h`: `3a0c15bb62b037aea2b0a6b4366452c4c057d841`
- `tests/unit/test_url_utils.c`: `cef8e0e69ea6a385cef14e926b105fba7970e987`

Изолированный повтор `ctest --test-dir /root/nvr-backend-verification-mbedtls367 --output-on-failure -R '^test_url_utils$'` вновь завершился с 1 fail из 16 assertions; повторился тот же `%3A`/`%3a` mismatch. Таким образом, suite failure воспроизводится при точном совпадении проверяемых test/implementation sources с HEAD и является pre-existing baseline limitation.
