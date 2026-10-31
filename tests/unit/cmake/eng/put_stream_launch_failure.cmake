add_layer3_test(test_put_stream_launch_failure)
target_link_options(test_put_stream_launch_failure PRIVATE
    "-Wl,--wrap=pthread_create" "-Wl,--wrap=free")
set_tests_properties(test_put_stream_launch_failure PROPERTIES TIMEOUT 15)
