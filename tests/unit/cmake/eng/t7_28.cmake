add_layer2_test(test_t7_28)
target_link_options(test_t7_28 PRIVATE
    -Wl,--wrap=db_event_route_count -Wl,--wrap=db_event_route_list
    -Wl,--wrap=db_event_route_generation -Wl,--wrap=db_event_destination_generation
    -Wl,--wrap=db_event_route_suppression_prune
    -Wl,--wrap=fopen -Wl,--wrap=stat
    -Wl,--wrap=calloc -Wl,--wrap=malloc -Wl,--wrap=free)
set_tests_properties(test_t7_28 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
