add_layer3_test(test_stream_state_disposal)
target_link_options(test_stream_state_disposal PRIVATE
    "-Wl,--wrap=pthread_mutex_destroy"
    "-Wl,--wrap=stream_stop_hls"
    "-Wl,--wrap=stop_mp4_recording"
    "-Wl,--wrap=reset_timestamp_tracker"
    "-Wl,--wrap=remove_timestamp_tracker")
set_tests_properties(test_stream_state_disposal PROPERTIES TIMEOUT 15)
