#!/usr/bin/env bash
# Чистая установка пользовательского форка lightNVR на Linux с Docker Compose v2.
set -Eeuo pipefail
trap 'echo "Ошибка установки в строке $LINENO. Данные сохранены; проверьте журнал Docker Compose." >&2' ERR

usage() {
  echo 'Использование: bash install-fork.sh [--dir /opt/nvr-fork] [--ref ветка_или_commit] [--bind 127.0.0.1] [--port 8080] [--resume-source-only]'
  echo 'Нужны Linux, Git, Docker Engine с Compose v2 и доступ к Docker daemon.'
}
install_dir=/opt/nvr-fork
source_ref=codex/mobile-live-favorites
web_bind=127.0.0.1
web_port=8080
resume_source_only=false
while (($#)); do
  case "$1" in
    --help|-h) usage; exit 0 ;;
    --resume-source-only) resume_source_only=true; shift ;;
    --dir|--ref|--bind|--port)
      (($# >= 2)) || { usage; exit 2; }
      case "$1" in
        --dir) install_dir=$2 ;;
        --ref) source_ref=$2 ;;
        --bind) web_bind=$2 ;;
        --port) web_port=$2 ;;
      esac
      shift 2 ;;
    *) usage; exit 2 ;;
  esac
done
[[ $(uname -s) == Linux ]] || { echo 'Скрипт предназначен для Linux.' >&2; exit 1; }
[[ $install_dir == /* && $install_dir != / && $install_dir != *$'\n'* ]] || { echo 'Укажите абсолютный каталог установки.' >&2; exit 2; }
[[ $web_bind =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo '--bind должен быть IPv4-адресом.' >&2; exit 2; }
[[ $web_port =~ ^[0-9]{1,5}$ ]] && ((10#$web_port >= 1 && 10#$web_port <= 65535)) || { echo 'Недопустимый порт.' >&2; exit 2; }
[[ -n $source_ref && $source_ref != -* ]] || { echo 'Недопустимый ref.' >&2; exit 2; }
for tool in git docker; do
  command -v "$tool" >/dev/null || { echo "Не установлена команда: $tool" >&2; exit 1; }
done
docker info >/dev/null
docker compose version >/dev/null
umask 077
if $resume_source_only; then
  [[ -d $install_dir/source/.git && ! -L $install_dir && ! -L $install_dir/source ]] || { echo 'Нет отдельного незавершённого клона source.' >&2; exit 1; }
  shopt -s nullglob dotglob
  entries=("$install_dir"/*)
  [[ ${#entries[@]} == 1 && ${entries[0]} == "$install_dir/source" ]] || { echo 'Продолжение разрешено только для каталога с одним source, без config/data.' >&2; exit 1; }
  [[ $(git -C "$install_dir/source" config --get remote.origin.url) == https://github.com/zirocool93/NVR.git ]] || { echo 'Клон принадлежит другому репозиторию.' >&2; exit 1; }
  git -C "$install_dir/source" fetch origin
else
  [[ ! -e $install_dir && ! -L $install_dir ]] || { echo 'Каталог уже существует. Чистая установка не перезаписывает данные.' >&2; exit 1; }
  mkdir -p "$install_dir"
  git clone --no-checkout https://github.com/zirocool93/NVR.git "$install_dir/source"
fi
install_dir=$(cd "$install_dir" && pwd -P)
# Разрешаем удалённую ветку до checkout: автоматический DWIM создаёт -b,
# несовместимый с --detach при первом checkout после --no-checkout clone.
if revision=$(git -C "$install_dir/source" rev-parse --verify "refs/remotes/origin/$source_ref^{commit}" 2>/dev/null); then
  :
else
  revision=$(git -C "$install_dir/source" rev-parse --verify "$source_ref^{commit}")
fi
git -C "$install_dir/source" checkout --detach "$revision"
git -C "$install_dir/source" submodule update --init --recursive
mkdir "$install_dir/config" "$install_dir/data"
cat > "$install_dir/compose.yaml" <<'YAML'
services:
  nvr:
    image: nvr-fork:${NVR_REVISION}
    build:
      context: ./source
      args:
        GIT_COMMIT: ${NVR_REVISION}
    restart: unless-stopped
    ports:
      - "${NVR_WEB_BIND}:${NVR_WEB_PORT}:8080"
      - "8554:8554"
      - "8555:8555"
      - "8555:8555/udp"
      - "127.0.0.1:1984:1984"
    volumes:
      - ./config:/etc/lightnvr
      - ./data:/var/lib/lightnvr/data
    environment:
      TZ: Europe/Moscow
      GO2RTC_CONFIG_PERSIST: "true"
      LIGHTNVR_AUTO_INIT: "true"
YAML
printf 'NVR_REVISION=%s\nNVR_WEB_BIND=%s\nNVR_WEB_PORT=%s\n' "$revision" "$web_bind" "$web_port" > "$install_dir/.env"
cd "$install_dir"
compose=(docker compose --project-name nvr-fork --project-directory "$install_dir" -f "$install_dir/compose.yaml")
"${compose[@]}" config --quiet
echo "Сборка форка, commit $revision. Первая сборка может занять длительное время."
"${compose[@]}" build --pull
"${compose[@]}" up -d
for ((attempt=0; attempt<120; attempt++)); do
  container_id=$("${compose[@]}" ps -q nvr)
  health=$(docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$container_id")
  if [[ $health == healthy ]]; then
    echo "Установка завершена. Web UI: http://$web_bind:$web_port"
    echo 'Первый вход: admin / admin. Сразу смените пароль перед внешним доступом.'
    echo "Commit: $revision; конфигурация и данные: $install_dir"
    exit 0
  fi
  [[ $health != unhealthy && $health != exited && $health != dead ]] || break
  sleep 5
done
"${compose[@]}" logs --tail 80 nvr
echo 'Контейнер не достиг healthy. Файлы установки сохранены для диагностики.' >&2
exit 1
