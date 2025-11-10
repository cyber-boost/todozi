from conan import ConanFile
from conan.tools.cmake import CMakeToolchain, CMakeDeps, CMake
from conan.tools.files import copy
import os

class TodoziConan(ConanFile):
    name = "todozi"
    version = "0.1.0"
    description = "A comprehensive task management system written in C"
    license = "MIT"  # Update with actual license
    author = "TonTon Bernie"
    topics = ("task-management", "productivity", "c", "library")

    # Binary configuration
    settings = "os", "compiler", "build_type", "arch"

    # Dependencies
    requires = [
        "jansson/2.14",
        "libcurl/8.4.0",
        "openssl/3.2.0",
        "util-linux-uuid/2.39.2"  # For libuuid
    ]

    # Optional dependencies for different features
    tool_requires = [
        "cmake/3.27.7"
    ]

    # Source and build info
    exports_sources = "src/*", "include/*", "CMakeLists.txt"

    def configure(self):
        # Header-only library for now, but could be static/shared
        self.options["jansson"].shared = False
        self.options["libcurl"].shared = False
        self.options["openssl"].shared = False

    def generate(self):
        # Generate CMake toolchain and dependencies
        tc = CMakeToolchain(self)
        tc.generate()

        deps = CMakeDeps(self)
        deps.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        # Copy headers
        copy(self, "*.h", self.source_folder, os.path.join(self.package_folder, "include"))

        # Copy libraries
        copy(self, "*.a", self.build_folder, os.path.join(self.package_folder, "lib"), keep_path=False)
        copy(self, "*.lib", self.build_folder, os.path.join(self.package_folder, "lib"), keep_path=False)

        # Copy executables if built
        copy(self, "todozi", self.build_folder, os.path.join(self.package_folder, "bin"), keep_path=False)

    def package_info(self):
        # Define library info for consumers
        self.cpp_info.libs = ["todozi"]
        self.cpp_info.includedirs = ["include"]

        # Define components for different modules
        self.cpp_info.components["core"].libs = ["todozi_core"]
        self.cpp_info.components["agent"].libs = ["todozi_agent"]
        self.cpp_info.components["api"].libs = ["todozi_api"]
        # ... add more components as needed
