add_layer2_test(test_api_handlers_system_health)
target_link_options(test_api_handlers_system_health PRIVATE
    "-Wl,--wrap=metrics_get_max_streams"
    "-Wl,--wrap=metrics_snapshot_all"
    "-Wl,--wrap=get_storage_health"
)
