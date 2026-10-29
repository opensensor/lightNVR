#!/usr/bin/env python3
"""Exercise the generated browser profile with real FFmpeg and go2rtc media.

No camera, credentials or GPU are required. All listeners bind to loopback and
all child processes are stopped on failure. See --help for standalone usage.
"""
import argparse
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import signal
import socket
import subprocess
import tempfile
import time
import urllib.request


def run(args, timeout=60, **kwargs):
    result = subprocess.run(args, capture_output=True, text=True, timeout=timeout, **kwargs)
    if result.returncode:
        raise AssertionError(f"Command failed ({result.returncode}): {args}\n{result.stderr[-6000:]}")
    return result.stdout


def port():
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


def probe(path):
    return json.loads(run(['ffprobe', '-v', 'error', '-show_entries',
                          'stream=codec_name,width,height,has_b_frames,pix_fmt',
                          '-of', 'json', str(path)]))['streams'][0]


def check_video(path, width, height, codec='h264'):
    info = probe(path)
    assert (info['codec_name'], info['width'], info['height']) == (codec, width, height), info
    if codec == 'h264':
        assert info['has_b_frames'] == 0, info
        assert info['pix_fmt'] == 'yuv420p', info
    # A valid header is not enough: decode real frames and fail on corruption.
    run(['ffmpeg', '-v', 'error', '-xerror', '-threads:v', '2', '-i', str(path),
         '-frames:v', '20', '-an', '-f', 'null', '-'])
    return info


def stop(process):
    if process and process.poll() is None:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait(timeout=5)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--config-writer', required=True)
    parser.add_argument('--guard', required=True)
    parser.add_argument('--go2rtc', default=os.environ.get('GO2RTC_TEST_BINARY'))
    args = parser.parse_args()
    if not shutil.which('ffmpeg') or not shutil.which('ffprobe'):
        print('SKIP: FFmpeg and ffprobe are required')
        return 77
    encoders = run(['ffmpeg', '-hide_banner', '-encoders'])
    if 'libx264' not in encoders or 'libx265' not in encoders:
        print('SKIP: libx264 and libx265 encoders are required')
        return 77

    with tempfile.TemporaryDirectory(prefix='lightnvr-live-transcode-') as tmp:
        root = Path(tmp)
        config_dir = root / 'config'
        run([args.config_writer, '--write-config', str(config_dir)])
        config = config_dir / 'go2rtc.yaml'
        match = re.search(r'^  h264: (".*")$', config.read_text(), re.MULTILINE)
        assert match, 'generated h264 template missing'
        template = shlex.split(json.loads(match[1]))
        assert '-threads:v 2' in config.read_text(), 'decoder workers must also be bounded'

        # Execute the real generated software template under a tighter RSS
        # budget than production. The old profile exceeds even 1536 MiB on a
        # many-core host before producing its first 7680x2160 video frame.
        for source, expected in [('7680x2160', (1920, 540)),
                                 ('3840x2160', (1920, 1080)),
                                 ('2160x3840', (608, 1080)),
                                 ('640x480', (640, 480)),
                                 ('641x481', (640, 480))]:
            output = root / f'{source}.mkv'
            pattern = (f'nullsrc=s={source}:r=20,format=yuv444p' if source == '641x481'
                       else f'color=s={source}:r=20')
            run([args.guard, '-v', 'error', '-threads:v', '2', '-f', 'lavfi', '-i',
                 pattern, '-frames:v', '30',
                 '-an', *template, str(output)],
                env={**os.environ, 'LIGHTNVR_FFMPEG_MAX_RSS_MB': '512'})
            print('PASS software preview', source, check_video(output, *expected), flush=True)

        if not args.go2rtc:
            print('SKIP loopback RTSP test: set GO2RTC_TEST_BINARY')
            return 0

        source = root / 'camera.mp4'
        run(['ffmpeg', '-v', 'error', '-f', 'lavfi', '-i', 'color=s=7680x2160:r=20',
             '-frames:v', '60', '-an', '-c:v', 'libx265', '-preset', 'ultrafast',
             '-x265-params', 'pools=2:frame-threads=2:keyint=20:bframes=0:log-level=error', str(source)])
        api_port, rtsp_port = port(), port()
        override = root / 'loopback.yaml'
        override.write_text(
            f'api:\n  listen: "127.0.0.1:{api_port}"\n  base_path: ""\n'
            f'rtsp:\n  listen: "127.0.0.1:{rtsp_port}"\n'
            'webrtc:\n  listen: ""\n'
            f'ffmpeg:\n  bin: {json.dumps(str(Path(args.guard).resolve()))}\n'
            # Deliberately fail hardware encoder startup even on GPU runners.
            '  h264/vaapi: "-c:v lightnvr_missing_encoder"\n'
            'streams:\n  camera: []\n  main:\n'
            f'    - "rtsp://127.0.0.1:{rtsp_port}/camera"\n'
            '    - "ffmpeg:main#video=h264#hardware=vaapi#width=1920#height=540"\n'
            '    - "ffmpeg:main#video=h264"\n'
        )
        processes = []
        with (root / 'go2rtc.log').open('w+') as log, (root / 'publisher.log').open('w+') as publog:
            try:
                server = subprocess.Popen([args.go2rtc, '-config', str(config), '-config', str(override)],
                                          stdout=log, stderr=log, start_new_session=True)
                processes.append(server)
                deadline = time.monotonic() + 10
                while True:
                    try:
                        with urllib.request.urlopen(f'http://127.0.0.1:{api_port}/api/streams', timeout=1):
                            break
                    except OSError:
                        if time.monotonic() >= deadline or server.poll() is not None:
                            raise AssertionError('go2rtc did not start')
                        time.sleep(0.1)
                publisher = subprocess.Popen(
                    ['ffmpeg', '-v', 'error', '-re', '-stream_loop', '-1', '-i', str(source),
                     '-an', '-c:v', 'copy', '-rtsp_transport', 'tcp', '-f', 'rtsp',
                     f'rtsp://127.0.0.1:{rtsp_port}/camera'],
                    stdout=publog, stderr=publog, start_new_session=True)
                processes.append(publisher)
                # Wait for the publisher's announced producer, not a fixed sleep.
                while True:
                    with urllib.request.urlopen(f'http://127.0.0.1:{api_port}/api/streams', timeout=1) as response:
                        data = json.load(response)
                    if data.get('camera', {}).get('producers'):
                        break
                    if time.monotonic() >= deadline or publisher.poll() is not None:
                        raise AssertionError('RTSP publisher did not attach')
                    time.sleep(0.1)

                # Original recording and compatibility preview run concurrently.
                consumers = []
                for codec in ('h265', 'h264'):
                    target = root / f'relay-{codec}.mkv'
                    process = subprocess.Popen(
                        ['ffmpeg', '-v', 'error', '-rtsp_transport', 'tcp', '-i',
                         f'rtsp://127.0.0.1:{rtsp_port}/main?video={codec}',
                         '-t', '2', '-an', '-c:v', 'copy', str(target)],
                        stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
                    processes.append(process)
                    consumers.append(process)
                for consumer in consumers:
                    _, stderr = consumer.communicate(timeout=45)
                    assert consumer.returncode == 0, stderr.decode(errors='replace')
                check_video(root / 'relay-h265.mkv', 7680, 2160, 'hevc')
                check_video(root / 'relay-h264.mkv', 1920, 540)
                print('PASS loopback RTSP: native 16MP HEVC recording + decoded H.264 preview, with failed hardware fallback', flush=True)
            except Exception:
                log.seek(0)
                publog.seek(0)
                print(log.read()[-8000:])
                print(publog.read()[-2000:])
                raise
            finally:
                for process in reversed(processes):
                    stop(process)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
