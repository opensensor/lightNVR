# SPDX-License-Identifier: GPL-3.0-or-later
"""The oracle must reject gaps, overlap and empty media sequences."""
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('media_recovery',Path(__file__).with_name('recovery.py'))
media = importlib.util.module_from_spec(spec)
spec.loader.exec_module(media)

class MediaAssertions(unittest.TestCase):
    def sequence(self):
        return [{'row':{'start_time':3*i,'end_time':3*(i+1)},
                 'first_two_hashes':[f'first-{i}',f'second-{i}'],
                 'last_two_hashes':[f'penultimate-{i}',f'last-{i}']} for i in range(3)]

    def test_consecutive_sequence_passes(self):
        media.check_sequence(self.sequence())

    def test_multiple_duplicate_boundary_frames_fail(self):
        rows=self.sequence();rows[1]['first_two_hashes']=rows[0]['last_two_hashes']
        with self.assertRaisesRegex(AssertionError,'duplicated'):
            media.check_sequence(rows)

    def test_one_shared_boundary_frame_is_allowed(self):
        rows=self.sequence();rows[1]['first_two_hashes'][0]=rows[0]['last_two_hashes'][-1]
        media.check_sequence(rows)

    def test_missing_segment_fails(self):
        rows=self.sequence();rows.pop(1)
        with self.assertRaises(AssertionError):media.check_sequence(rows)

    def test_unexpected_gap_fails(self):
        rows=self.sequence();rows[1]['row']['start_time']+=2
        with self.assertRaises(AssertionError):media.check_sequence(rows)

    def test_overlap_fails(self):
        rows=self.sequence();rows[1]['row']['start_time']-=2
        with self.assertRaises(AssertionError):media.check_sequence(rows)

    def test_empty_sequence_cannot_pass(self):
        with self.assertRaises(AssertionError):media.check_sequence([])

if __name__=='__main__':unittest.main()
