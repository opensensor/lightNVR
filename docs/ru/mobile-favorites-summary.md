# Итог итерации: мобильный Live View и персональное избранное

Дата: 02.10.2026. Репозиторий: https://github.com/zirocool93/NVR. Рабочая ветка: `codex/mobile-live-favorites`. База: `03767df2d6cbebfa388be123fa5d305a33698674` (0.42.9). Изменения локальные: коммит, push, PR и обновление действующего сервера не выполнялись.

## Реализовано

Главная страница выбирает интерфейс по персональному `ui_mode`: auto (по умолчанию), mobile, desktop. Настройка хранится на сервере; до её загрузки проигрыватели не монтируются. Auto использует ширину <=760px либо ширину <=1024px с coarse pointer. Development override действует только в development сборке.

Мобильный Live View содержит поиск, вкладки «Все»/«Избранное», звёзды, состояния загрузки/ошибки/пустого списка, полноэкранный просмотр и touch targets не менее 44px. Страница содержит максимум четыре карточки; IntersectionObserver ограничивает фактическое число проигрывателей. Список камер и избранное обновляются каждые 30 секунд. Избранное привязано к стабильному camera_uuid, сортируется по числовому created_at и UUID; удаление последней карточки корректирует страницу.

`UiModeControl.jsx` выбирает новый `MobileLiveView.jsx` с внутренним MobileCameraCard либо прежний desktop. Используется существующий PlaybackTransportCell. В WebRTCVideoCell добавлены точечные проверки отмены после await и закрытие позднего соединения: браузерный стресс-тест обнаружил исходную утечку при unmount во время ICE-запроса. Recorder, go2rtc, конфигурация потоков и транспортная сигнализация не менялись.

## Миграция и API

Миграция `0088_user_ui_preferences_favorites.sql` добавляет user_preferences и user_favorites с внешними ключами и каскадным удалением. Значение auto представлено отсутствием настройки. Избранное имеет уникальную пару (user_id, camera_uuid); повторное добавление сохраняет created_at. Встроенный migration header обновлён добавлением 0088.

| Метод и путь | Назначение |
|---|---|
| GET /api/ui/preferences | Режим текущего пользователя и configurable |
| PUT /api/ui/preferences | Сохранить auto/mobile/desktop |
| GET /api/ui/favorites | Доступное текущему пользователю избранное |
| POST /api/ui/favorites | Идемпотентно добавить camera_uuid |
| DELETE /api/ui/favorites/:camera_uuid | Идемпотентно удалить собственную запись |

Пользователь определяется существующей авторизацией, user_id в запросе отвергается. Чтение камер использует существующее AUTHZ_LIVE_VIEW. Неавторизованный запрос возвращает 401; demo/scoped/no-user имеют read-only значения и запрет записи 403. Недоступная либо отсутствующая камера при добавлении возвращает одинаковый 404. Роли и модель доступа не переработаны. Query cache разделён по серверному идентификатору пользователя.

## Проверки и результаты

| Проверка | Результат |
|---|---|
| Frontend production/legacy build | Пройдено |
| Jest, TZ=America/New_York | 41 suite, 311/311 |
| Backend CMake build в WSL Ubuntu | Пройдено; SOD/LiteRT отключены только для тестовой сборки |
| CTest | 144/145; новый API test пройден |
| Генератор миграций | --check и 4 теста пройдены |
| Chromium: телефон, landscape, desktop, темы, fullscreen | Пройдено |
| 80 камер, 20 страниц вперёд/назад, задержанный ICE | Максимум 4 активных PC; после стресс-прогона 166 создано, 163 закрыто, 3 активно |
| Upgrade тестовой БД 0087 → 0088 | Миграция и вход существующего пользователя пройдены |
| Реальные cookie-сессии A/B | Раздельные режимы/избранное, идемпотентность, logout/login и перезапуск пройдены |

Единственный сбой CTest — исходный test_url_utils: ожидание %3A против %3a. Неизменность реализации и теста относительно базового commit и повторный изолированный сбой подтверждены. Полный CTest поэтому не является полностью зелёным. Существующие DST fixtures Jest требуют America/New_York; в UTC/МСК пять таких проверок падают.

Браузерная проверка использовала настоящие frontend-компоненты и RTCPeerConnection, но заглушки HTTP/SDP: декодирование реального видео не проверено. Исходное предупреждение Vite о размере LiveView chunk сохраняется. Legacy hls.html и редактирование режима другого пользователя администратором отложены. Тестовые серверы и preview остановлены.

## Документы и дальнейший шаг

- [Журнал решений](mobile-live-favorites.md)
- [Backend: схема и API](mobile-favorites-backend.md)
- [Frontend: компоненты и жизненный цикл](mobile-favorites-frontend.md)
- [Интеграционные проверки](mobile-favorites-validation.md)
- [Сборка и исходный сбой CTest](backend-build-validation.md)
- [Среда и команды сборки](build-environment.md)
- [Runtime-проверка пользователей A/B](runtime-user-validation.md)

Рекомендуемый следующий отдельный этап — собрать тестовый образ из этой ветки и проверить реальные камеры, воспроизведение и миграцию на копии серверных данных перед обновлением production. Этот этап автоматически не начат.

## Полный список изменённых файлов

- `include/database/db_embedded_migrations.h`
- `src/web/libuv_api_handlers.c`
- `tests/unit/CMakeLists.txt`
- `web/css/live.css`
- `web/js/components/preact/AuthGate.jsx`
- `web/js/components/preact/WebRTCVideoCell.jsx`
- `web/js/pages/index-page.jsx`
- `web/js/utils/auth-utils.js`
- `db/migrations/0088_user_ui_preferences_favorites.sql`
- `docs/ru/backend-build-validation.md`
- `docs/ru/build-environment.md`
- `docs/ru/mobile-favorites-backend.md`
- `docs/ru/mobile-favorites-frontend.md`
- `docs/ru/mobile-favorites-summary.md`
- `docs/ru/mobile-favorites-validation.md`
- `docs/ru/mobile-live-favorites.md`
- `docs/ru/runtime-user-validation.md`
- `include/database/db_ui_preferences.h`
- `include/web/api_handlers_ui_preferences.h`
- `src/database/db_ui_preferences.c`
- `src/web/api_handlers_ui_preferences.c`
- `tests/unit/test_api_handlers_ui_preferences.c`
- `web/js/components/preact/MobileLiveView.jsx`
- `web/js/components/preact/UiModeControl.jsx`
- `web/js/utils/mobile-ui.js`
- `web/tests/e2e/mobile-favorites.browser.cjs`
- `web/tests/mobile-ui.spec.js`

## Дополнение: публикация
Ветка опубликована: https://github.com/zirocool93/NVR/tree/codex/mobile-live-favorites. Implementation commit: 27f268c01f01878a23811baa9f3ac60fbe30077d. Первоначальная отметка о локальном состоянии выше относится к завершению предыдущей итерации. Добавлены scripts/install-fork.sh, docs/ru/clean-install.md и tests/unit/test_install_fork.py. Инструкция: [Чистая установка](clean-install.md). Для повторения изолированной проверки: python3 tests/unit/test_install_fork.py на Linux. Скрипт синтаксически проверен; реальные Docker build/up ещё не проверены. Основная ветка и действующий сервер не изменены.
