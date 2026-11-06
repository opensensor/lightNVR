"""Ensure clean-checkout delivery applies the fix, rather than merely exiting zero."""
import hashlib
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SOURCE = Path('internal/http/http.go')
TEST = Path('internal/http/http_test.go')
PIN = '6986a1b3f122651b318a957534da44bb4c65e271'


class Go2RtcPatchTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.pristine = subprocess.check_output(
            ['git', '-C', str(ROOT / 'go2rtc'), 'show', f'{PIN}:{SOURCE}'])

    def apply(self, directory):
        return subprocess.run(['sh', str(ROOT / 'scripts/apply-go2rtc-patches.sh'),
                               str(directory)], capture_output=True, text=True)

    @staticmethod
    def files(directory):
        return {str(p.relative_to(directory)): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in directory.rglob('*') if p.is_file()}

    def test_pristine_copy_applies_and_second_application_preserves_edits(self):
        with tempfile.TemporaryDirectory(prefix='lightnvr-go-patch-') as temporary:
            directory = Path(temporary)
            target = directory / SOURCE
            target.parent.mkdir(parents=True)
            target.write_bytes(self.pristine + b'\n// unrelated local edit\n')
            result = self.apply(directory)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn(b'if res.StatusCode != http.StatusOK {\n\t\ttcp.Close(res)',
                          target.read_bytes())
            self.assertTrue(target.read_bytes().endswith(b'// unrelated local edit\n'))
            self.assertIn('func TestRejectedProducerClosesResponseBody',
                          (directory / TEST).read_text())
            before = self.files(directory)
            result = self.apply(directory)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(before, self.files(directory))

    def test_incompatible_copy_fails_before_writing_any_file(self):
        with tempfile.TemporaryDirectory(prefix='lightnvr-go-mismatch-') as temporary:
            directory = Path(temporary)
            target = directory / SOURCE
            target.parent.mkdir(parents=True)
            target.write_bytes(self.pristine.replace(
                b'if res.StatusCode != http.StatusOK {',
                b'if res.StatusCode != http.StatusAccepted {'))
            before = self.files(directory)
            result = self.apply(directory)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(before, self.files(directory))

    def test_partly_applied_copy_requires_review_without_completing_it(self):
        with tempfile.TemporaryDirectory(prefix='lightnvr-go-partial-') as temporary:
            directory = Path(temporary)
            target = directory / SOURCE
            target.parent.mkdir(parents=True)
            target.write_bytes(self.pristine.replace(
                b'if res.StatusCode != http.StatusOK {',
                b'if res.StatusCode != http.StatusOK {\n\t\ttcp.Close(res)'))
            before = self.files(directory)
            result = self.apply(directory)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(before, self.files(directory))


if __name__ == '__main__':
    unittest.main()
