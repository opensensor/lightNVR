# T27 preserves all existing media tests and declares bounded native deadlines.
set_tests_properties(test_mp4_audio_recovery test_mp4_recovery_watchdog
    test_mp4_writer_watchdog test_mp4_segment_boundaries test_mp4_segment_timestamps
    test_mp4_faststart PROPERTIES TIMEOUT 30 LABELS "engineering;media")
