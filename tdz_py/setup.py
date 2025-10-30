#!/usr/bin/env python3

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="todozi",
    version="0.1.0",
    author="CyberBoost",
    description="AI/Human task management system with file-based storage",
    long_description=open("../README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/cyber-boost/todozi",
    packages=["todozi"],
    rust_extensions=[RustExtension("todozi._todozi", binding=Binding.PyO3)],
    zip_safe=False,
    python_requires=">=3.8",
    install_requires=[
        "textual>=0.19.0",
        "rich>=13.0.0",
    ],
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: System :: Shells",
        "Topic :: Utilities",
    ],
    entry_points={
        "console_scripts": [
            "todozi-tui=todozi.tui:main",
            "todozi-cli=todozi.main:main",
        ],
    },
)
