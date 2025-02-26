cmake_minimum_required(VERSION 3.28)

project(
  person
  VERSION 0.1.0
  LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(${PROJECT_NAME} STATIC ${CMAKE_CURRENT_SOURCE_DIR}/src/person.cpp)
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

install(TARGETS ${PROJECT_NAME} DESTINATION .)

# Copy the compile commands to cargos target directory ##
set(CC_TARGET_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../target")
set(CC_FILE "${CMAKE_CURRENT_BINARY_DIR}/compile_commands.json")

add_custom_command(
  OUTPUT ${CC_TARGET_DIR}/compile_commands.json
  COMMAND ${CMAKE_COMMAND} -E copy ${CC_FILE}
          ${CC_TARGET_DIR}/compile_commands.json
  DEPENDS ${CC_FILE}
  COMMENT "Copying generated compile_commands.json to destination")

add_custom_target(copy_compile_commands ALL
                  DEPENDS ${CC_TARGET_DIR}/compile_commands.json)
