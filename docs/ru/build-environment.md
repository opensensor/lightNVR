# Локальная среда сборки LightNVR

## Состояние на 2026-10-02

WSL Ubuntu подготовлена для локальной backend сборки. Репозиторий приложения не менялся для подготовки окружения, production-код не изменён. Штатная сборка LightNVR и тесты ещё не запускались: их должен выполнить backend/frontend worker после завершения изменений.

- Windows Node.js `v24.20.0`, npm `11.19.0` подходят требованию README `>=24.11 <25` для web toolchain.
- Ubuntu WSL: GCC `13.3.0`, CMake `3.28.3`, pkg-config, SQLite, cJSON, curl, mbedTLS, Mosquitto, YAML, libuv, OpenSSL и dev packages установлены. `dpkg --audit` и `apt-get check` прошли без ошибок.
- Docker CLI установлен, но daemon недоступен: отсутствуют pipe `docker_engine` и `dockerDesktopLinuxEngine`.

## FFmpeg и llhttp для CMake

Дистрибутивный Ubuntu 24.04 FFmpeg 6.1 не подходит требованиям CMake проекта. Поэтому официальный исходный архив FFmpeg 7.1 (`https://ffmpeg.org/releases/ffmpeg-7.1.tar.xz`) скачан через Windows Node в staging. SHA-256 архива: `40973D44970DBC83EF302B0609F2E74982BE2D85916DD2EE7472D30678A7ABE6`.

FFmpeg собран в WSL с shared libraries, без static libraries, программ, документации и x86 assembly; остальные штатные codec/demuxer/muxer features не отключались. Установка находится только в тестовом prefix `/opt/lightnvr-test-deps`, системный FFmpeg не заменён. Проверенные `pkg-config --modversion` значения:

| Библиотека | Версия |
|---|---:|
| libavcodec | 61.19.100 |
| libavformat | 61.7.100 |
| libavutil | 59.39.100 |
| libswscale | 8.3.100 |
| libswresample | 5.3.100 |

Для CMake, запуска собранных тестов и LightNVR в WSL выставлять:

```bash
export PKG_CONFIG_PATH=/opt/lightnvr-test-deps/lib/pkgconfig
export LD_LIBRARY_PATH=/opt/lightnvr-test-deps/lib
```

llhttp `release/v9.2.1`, commit `610a87d755f6bae466cd871c2ba97574ccac5483`, склонирован Windows Git вне репозитория в `C:\Users\sumin\Documents\ChatGPT\flussonic\work\build-deps\llhttp-src`, собран WSL CMake как shared library и установлен в тот же prefix. `pkg-config --modversion libllhttp` возвращает `9.2.1`. С указанным `PKG_CONFIG_PATH` проектный CMake найдёт llhttp через `pkg_check_modules(LLHTTP QUIET libllhttp)` и не должен загружать raw-файлы в `external/llhttp`.

## Как подготовлены WSL apt packages

WSL Ubuntu видела apt package lists, но прямой сетевой доступ из WSL таймаутился. На время операции запущен Windows Node HTTP forward proxy, привязанный только к WSL host-adapter `172.18.64.1:18080`. С ним apt indexes успешно обновились. Proxy остановлен после загрузки; persistent proxy, WSL и firewall настройки не менялись.

По свежим индексам `apt-get --print-uris --yes --no-install-recommends install ...` разрешил 183 архива общим размером `174,926,346` байт. Они скачаны в staging вне репозитория:

`C:\Users\sumin\Documents\ChatGPT\flussonic\work\build-deps\deb-current`

Windows Node проверил каждый пакет по MD5, выданному apt для соответствующего URI. В WSL архивы установлены через `dpkg -i`; проверка package database прошла.

FFmpeg 7.1 архив и исходники llhttp хранятся рядом в `work/build-deps`. Кэшированные `.deb` подходят этой Ubuntu 24.04 WSL и могут повторно использоваться при необходимости.

## Проверенные требования и ограничения

- `README.md` задаёт Node.js диапазон и актуальные требования к FFmpeg.
- `CMakeLists.txt` задаёт минимальные версии FFmpeg и сначала ищет `libllhttp` через pkg-config; fallback CMake download пишет в `external/llhttp`.
- `docs/BUILD.md` описывает штатные команды сборки.
- Полная сборка и integration tests намеренно не запускались до завершения параллельных изменений.
