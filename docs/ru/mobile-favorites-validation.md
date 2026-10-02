# Результаты проверки мобильного Live View и избранного

Дата: 02.10.2026  
Ветка: `codex/mobile-live-favorites`  
Проверялась локальная сборка и отдельный browser harness. Действующий сервер не запускался и не менялся.

## Frontend

`npm run build` завершился успешно после исправления цветов темы и размеров видеокарточек. Сборка включила обычный и legacy targets Vite; остались обычные предупреждения о размере крупного Live View chunk.

Полный Jest suite прошёл при часовом поясе `America/New_York`: 41 suite, 311 тестов. Команда из `web`:

```powershell
$env:TZ='America/New_York'
npm test -- --runInBand
```

Пять тестов DST в `timeline-utils.spec.js` падают, если запускать suite с `TZ=Europe/Moscow` или UTC: они намеренно проверяют календарные дни Нью-Йорка, а тестируемая логика зависит от локального часового пояса процесса. С `America/New_York` те же тесты проходят. В этом окружении изменения мобильного режима и избранного их не затрагивают.

## Проверка в Chromium

Добавлен повторно используемый сценарий `web/tests/e2e/mobile-favorites.browser.cjs`. Он открывает production preview на `http://127.0.0.1:4173/`, подменяет API ответами 80 тестовых камер и перехватывает go2rtc WebRTC endpoint. Браузер создаёт реальные `WebRTCVideoCell`, `RTCPeerConnection` и video elements; SDP endpoint отвечает тестовым SDP и не подключается к камерам. Поэтому результат проверяет frontend mount/cleanup, а не получение настоящего видео или backend.

В Windows окружении с установленным Playwright сценарий запускается из `web` так:

```powershell
$env:PLAYWRIGHT_MODULE='C:\Users\sumin\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\node_modules\playwright'
$env:MOBILE_BROWSER_ARTIFACTS='C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile'
node tests/e2e/mobile-favorites.browser.cjs
```

Сценарий проверяет 390×844 portrait, 844×390 landscape, explicit mobile на 1440×900, auto desktop на 1440×900 и 1440×1080, а также автоматический coarse-pointer выбор. Он проверяет задержку preferences до первого mount, не более четырёх карточек/video elements и активных RTCPeerConnection, сохранение/удаление звезды после reload, сортировку числового Unix `created_at`, скрытие недоступной камеры, отображение API ошибки, clamp последней страницы после удаления, очистку fullscreen при unmount, отсутствие горизонтальной прокрутки и цветовой контраст в light/dark темах.

Последний Chromium прогон прошёл. При первом старте мобильная страница показала три video elements и три активных RTCPeerConnection для 80 камер. После смены страницы в 300 мс и в 2 секунды оставалось три активных соединения, максимум одновременно — четыре. Проверка 20 страниц туда и обратно создала 166 соединений, закрыла 163 и оставила три активных; задержанный ответ `/api/ice-servers` после ухода со страницы оставил три активных соединения для трёх текущих video elements. Instrumentation учитывает повторный `close()` через `WeakSet`; после ICE сценария `created - closed` совпало с `active`.

На 1440×900 в explicit mobile видеокарточка имеет высоту 466.9 px, плеер и video element — 412.9 px; схлопывание плеера до нуля больше не воспроизводится. На landscape 844×390 `scrollWidth` равен viewport width: 844 px. Auto coarse-pointer сценарий на 932×430 прошёл после явной CDP touch-эмуляции: 4 карточки. Контраст выбранной вкладки — 6.03:1 в светлой и 7.03:1 в тёмной теме.

Desktop auto режим показал 36 ячеек сетки. При 1440×900 панель режима находится на y=80…140 px, сетка — y=212…670.1 px, footer — y=856…900 px. При высоте 1080 px footer заканчивается на нижней границе viewport; обрезанного контента не обнаружено.

Скриншоты и JSON последнего результата сохранены вне репозитория:

- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\01-phone-portrait.png`
- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\02-explicit-mobile-desktop-viewport.png`
- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\03-desktop-auto-1440x900.png`
- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\04-desktop-auto-1440x1080.png`
- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\05-phone-landscape.png`
- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\06-theme-dark.png`
- `C:\Users\sumin\Documents\ChatGPT\flussonic\outputs\nvr-mobile\results.json`

## Backend

В `tests/unit/test_api_handlers_ui_preferences.c` добавлены проверки раздельного `ui_mode` пользователей A/B, запрета подмены `user_id`, ответа 404 на валидный UUID отсутствующей камеры, удаления только избранного текущего пользователя и сохранения избранного по UUID после переименования камеры. Тест каскадного удаления user preferences/favorites и удаления камеры уже был в suite.

Backend собирался в WSL Ubuntu с отдельными test-only FFmpeg 7.1, llhttp 9.2.1 и mbedTLS 3.6.7; production source для адаптации системных зависимостей не менялся. Полный CTest завершился с результатом **144/145**. Целевой `test_api_handlers_ui_preferences` прошёл. Единственный сбой — существующий `test_url_utils`: в `test_url_apply_credentials_replaces_existing_credentials` тест ожидает `%3A`, реализация выдаёт `%3a`; это не связано с UI preferences/favorites.

Проведён отдельный runtime API smoke test на тестовом сервере и базе `/tmp/lightnvr-mobile-integration`, порт 18081. Basic Auth `admin/admin` вернул `/api/auth/verify` 200; GET/PUT `/api/ui/preferences` вернули 200, GET `/api/ui/favorites` — 200, POST валидного UUID камеры, которой нет в БД, — 404. Чтобы проверить upgrade, тестовая база была приведена к schema head `0087`: удалены только таблицы `user_preferences`/`user_favorites` и запись migration `0088`. Повторный запуск применил `0088_user_ui_preferences_favorites`, head стал `0088`; строка пользователя admin сохранилась и повторная проверка входа вернула 200. Таблицы обе созданы. Это проверка миграции и сохранения логина существующего пользователя; полноценный сценарий с двумя cookie-сессиями и добавлением избранной реальной камеры выполнен отдельным backend runtime harness и описан в runtime-user-validation.md.

Отдельный backend runtime прогон с настоящими cookie-сессиями выполнен на тестовой БД `/tmp/nvr-auth-favorites-runtime/data/lightnvr.db` и порту 18082. Пользователь A сохранил `mobile`, пользователь B — `desktop`; новая допустимая, но отсутствующая камера вернула 404, spoof `user_id` в запросах preferences/favorites — 400. A добавил камеру в избранное; повторный POST через 1.1 секунды не изменил `created_at`. У B список избранного остался пустым, а DELETE от B не удалил запись A. `POST /api/auth/logout` вернул 302 и удалил сессию; новый login A и B после перезапуска вернул 200. A увидел `mobile` и 1 избранное, B — `desktop` и 0. Тестовая камера отключена, оба тестовых процесса на 18081/18082 остановлены. Полные команды и HTTP результаты приведены в [runtime-user-validation.md](runtime-user-validation.md).

Первая попытка запуска из Windows-mount каталога остановилась: Linux увидел migration SQL как world-writable и отказался исполнять файл. Повторный тест использовал копию migration files в изолированном `/tmp` с режимами каталогов 755 и SQL 644. В production файлы и разрешения не менялись.

Полный backend suite запускался так:

```bash
export PKG_CONFIG_PATH=/opt/lightnvr-test-deps/lib/pkgconfig
export LD_LIBRARY_PATH=/opt/lightnvr-test-deps/lib
export LIBRARY_PATH=/opt/lightnvr-test-deps/lib
export CMAKE_PREFIX_PATH=/opt/lightnvr-test-deps
ctest --test-dir /root/nvr-backend-verification-mbedtls367 --output-on-failure
```

Для runtime migration проверки бинарь `/root/nvr-backend-verification-mbedtls367/bin/lightnvr` использовал отдельный config и каталог `/tmp/lightnvr-mobile-integration`; сервер запускался в WSL Ubuntu на `18081`, после проверки тестовый процесс останавливался. Боевой сервер и его данные не использовались.

