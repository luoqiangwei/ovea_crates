cmake_minimum_required(VERSION 3.10)
project(my_math_lib C)

# Set include directory
include_directories(include)

add_library(my_math STATIC src/my_math.c)
add_library(my_math_dyn SHARED src/my_math.c)
set(CMAKE_MACOSX_RPATH 1)

# 1. Create Static Library (.a / .lib)
# add_library(my_math_static STATIC src/my_math.c)
# set_target_properties(my_math_static PROPERTIES OUTPUT_NAME "my_math")

# 2. Create Dynamic Library (.so / .dll)
# add_library(my_math_shared SHARED src/my_math.c)
# set_target_properties(my_math_shared PROPERTIES OUTPUT_NAME "my_math")

# install(TARGETS my_math_static my_math_shared
#         ARCHIVE DESTINATION lib    # 静态库 (.a) 安装到 lib 目录
#         LIBRARY DESTINATION lib    # 动态库 (.so/.dylib) 安装到 lib 目录
#         # RUNTIME DESTINATION bin    # Windows 上的 DLL 安装到 bin 目录
# )

install(TARGETS my_math my_math_dyn
        ARCHIVE DESTINATION lib  # 静态库
        LIBRARY DESTINATION lib  # 动态库
)

install(FILES include/my_math.h DESTINATION include)
