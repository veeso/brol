cmake_minimum_required(VERSION 3.0)
project(libeasyfb VERSION 0.1.0)

set (ROOT_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src/")
set (ROOT_TESTS_DIR "${CMAKE_CURRENT_SOURCE_DIR}/tests/")
include_directories(
  "${CMAKE_CURRENT_SOURCE_DIR}/include/"
)
#Includes
file(GLOB INCLUDE_FILES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/easyfb/*.h"
)

#Library
file(GLOB EASYFB_SRC
  "${ROOT_SOURCE_DIR}/*.c"
)

#Tests source
file(GLOB WRITE_TEST_SRC
  "${ROOT_TESTS_DIR}/test-write/*.c"
)
file(GLOB READ_TEST_SRC
  "${ROOT_TESTS_DIR}/test-read/*.c"
)
file(GLOB CHESS_TEST_SRC
  "${ROOT_TESTS_DIR}/test-chess/*.c"
)

set(CXX_FLAGS "-g -Wall")
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -Wall")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -Wall")
set(CMAKE_C_STANDARD 11)
set(CMAKE_BUILD_TYPE CMAKE_CXX_FLAGS_DEBUG)

add_library(easyfb_shared SHARED ${EASYFB_SRC})
set_target_properties(easyfb_shared PROPERTIES OUTPUT_NAME easyfb)
set_target_properties(easyfb_shared PROPERTIES VERSION 0.1.0)
add_library(easyfb_static STATIC ${EASYFB_SRC})
set_target_properties(easyfb_static PROPERTIES OUTPUT_NAME easyfb)

#Build tests
add_executable(easyfb-write ${WRITE_TEST_SRC})
target_link_libraries(easyfb-write easyfb_shared)
add_executable(easyfb-chess ${CHESS_TEST_SRC})
target_link_libraries(easyfb-chess easyfb_shared)
add_executable(easyfb-read ${READ_TEST_SRC})
target_link_libraries(easyfb-read easyfb_shared)

if(CMAKE_COMPILER_IS_GNUCXX)
  target_compile_options(easyfb-read PUBLIC "--coverage")
  target_link_libraries(easyfb-read gcov)
  target_compile_options(easyfb-write PUBLIC "--coverage")
  target_link_libraries(easyfb-write gcov)
endif()
#Install rules
install(TARGETS easyfb_shared CONFIGURATIONS Release LIBRARY DESTINATION lib PUBLIC_HEADER DESTINATION include)
install(TARGETS easyfb_static CONFIGURATIONS Release ARCHIVE DESTINATION lib)
INSTALL(FILES ${INCLUDE_FILES} DESTINATION include/easyfb/)
