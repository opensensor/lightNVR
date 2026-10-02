# Проверка пользовательских настроек и избранного по HTTP

Проверка выполнена на локальном WSL Ubuntu с бинарным файлом `/root/nvr-backend-verification-mbedtls367/bin/lightnvr`. Использовались отдельные конфигурация, БД и каталог хранения в `/tmp/nvr-auth-favorites-runtime`; HTTP слушал порт `18082`. Порт `18081` и fixture integration-тестера не затрагивались. Сервер запускался с копией `db/migrations` из репозитория и отключённым go2rtc. Камера создана выключенной, поэтому подключение к RTSP-источнику не выполнялось.

Команды запуска из WSL:

```bash
mkdir -p /tmp/nvr-auth-favorites-runtime/source/db /tmp/nvr-auth-favorites-runtime/data
cp -a /mnt/c/Users/sumin/Documents/ChatGPT/flussonic/work/nvr-fork/db/migrations \
  /tmp/nvr-auth-favorites-runtime/source/db/
cd /tmp/nvr-auth-favorites-runtime/source
export LD_LIBRARY_PATH=/opt/lightnvr-test-deps/lib
/root/nvr-backend-verification-mbedtls367/bin/lightnvr \
  -c /tmp/nvr-auth-favorites-runtime/config.ini
```

Конфигурация использовала `auth_enabled = true`, `username = admin`, `password = admin`, порт `18082`, БД `/tmp/nvr-auth-favorites-runtime/data/lightnvr.db` и существующую собранную веб-папку. HTTP-сценарий выполнен внешним временным проверочным скриптом в `work/build-deps`; скрипты bootstrap/runtime не добавлялись в репозиторий.

Через `POST /api/auth/login` созданы реальные cookie-сессии администратора и двух пользователей (`runtimeUserA`, `runtimeUserB`). Пользователи созданы штатным `POST /api/auth/users` с ролью viewer. Выполнены следующие проверки:

- У нового пользователя A `GET /api/ui/preferences` вернул `ui_mode: auto`. После `PUT` пользователь A получил `mobile`, пользователь B — `desktop`.
- Неизвестная, но синтаксически корректная UUID камеры при `POST /api/ui/favorites` вернула `404`. Добавление поля `user_id` в тело изменения preference или добавления favorite вернуло `400`.
- Администратор создал выключенную тестовую камеру `runtime-fav-camera` через `POST /api/streams`; сервер назначил UUID `56b17311-a867-4250-89c3-ab0830dd40b4`.
- A добавил камеру в избранное. Повторный `POST` после паузы 1,1 секунды вернул прежний `created_at` (`1790962455`), то есть дубликат не перезаписал время создания.
- У B список избранного оставался пустым. `DELETE /api/ui/favorites/{camera_uuid}` из cookie-сессии B вернул `200`, но элемент A сохранился.
- `POST /api/auth/logout` для A завершился `302` после удаления серверной сессии; новый login A вернул сохранённые `mobile` и favorite.
- После остановки и нового запуска сервера на той же БД успешный login A вернул `mobile` и один favorite; login B вернул `desktop` и пустой список.

Контрольное чтение SQLite после перезапуска показало `user_preferences`: `(2, ui_mode, "mobile")`, `(3, ui_mode, "desktop")`; `user_favorites`: `(2, 56b17311-a867-4250-89c3-ab0830dd40b4, 1790962455)`. Текущая схема хранит расширяемые настройки в `user_preferences(user_id, preference_key, value_json, updated_at)` и избранное в `user_favorites(user_id, camera_uuid, created_at)`.

Сервер остановлен; проверка `ss -ltn 'sport = :18082'` не показала слушателя. Данные и бинарные зависимости оставлены только в WSL временном каталоге для возможной повторной диагностики.
