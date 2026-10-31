add_layer2_test(test_t7_53)
target_link_options(test_t7_53 PRIVATE
    -Wl,--wrap=realloc
    -Wl,--wrap=db_authorization_load_user_grants
    -Wl,--wrap=camera_collection_filter_load_for_authorization
    -Wl,--wrap=camera_collection_filter_matches
    -Wl,--wrap=camera_collection_filter_free)
set_tests_properties(test_t7_53 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
