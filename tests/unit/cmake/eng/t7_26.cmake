add_layer2_test(test_t7_26)
target_link_options(test_t7_26 PRIVATE -Wl,--wrap=lstat -Wl,--wrap=syscall -Wl,--wrap=unlink -Wl,--wrap=db_storage_migration_update_progress -Wl,--wrap=db_storage_migration_commit_location)
set_tests_properties(test_t7_26 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
