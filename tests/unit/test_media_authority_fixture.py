#!/usr/bin/env python3
"""Generate an owned video+tone segment and execute native media-policy tests."""
import pathlib
import subprocess
import sys
import tempfile
with tempfile.TemporaryDirectory(prefix='lightnvr-media-authority-') as directory:
    source = pathlib.Path(directory) / 'av.ts'
    subprocess.run(['ffmpeg', '-hide_banner', '-loglevel', 'error', '-f', 'lavfi',
                    '-i', 'testsrc=size=160x120:rate=10', '-f', 'lavfi', '-i',
                    'sine=frequency=880:sample_rate=48000', '-t', '1', '-c:v',
                    'libx264', '-preset', 'ultrafast', '-pix_fmt', 'yuv420p', '-g', '10', '-c:a', 'aac', '-f', 'mpegts', str(source)], check=True)
    subprocess.run([sys.argv[1], str(source)], check=True)
