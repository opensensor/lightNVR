add_executable(test_t7_2_migrations ${CMAKE_SOURCE_DIR}/tests/unit/test_t7_2_migrations.c)
target_include_directories(test_t7_2_migrations PRIVATE ${CMAKE_SOURCE_DIR}/include ${FFMPEG_INCLUDE_DIRS})
target_link_libraries(test_t7_2_migrations PRIVATE unity_lib lightnvr_lib
    ${FFMPEG_LIBRARIES} ${SQLITE_LIBRARIES} ${CURL_LIBRARIES} ${SSL_LIBRARIES}
    ${HTTP_BACKEND_LIBS} inih_lib pthread dl m)
add_test(NAME test_t7_2_migrations COMMAND test_t7_2_migrations)
set_tests_properties(test_t7_2_migrations PROPERTIES TIMEOUT 15 LABELS "engineering;security")
