#!/usr/bin/env python3

from setuptools import setup, find_packages

with open("./README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="todozi",
    version="0.1.0",
    author="TonTon Bernie",
    description="AI/Human task management system with file-based storage",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/cyber-boost/todozi",
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    python_requires=">=3.8",
    install_requires=[
        "fastapi>=0.111.0",
        "uvicorn[standard]>=0.30.0",
        "sqlmodel>=0.0.16",
        "pydantic>=2.6.0",
        "pydantic-settings>=2.3.0",
        "python-multipart>=0.0.9",
        "torch>=2.0.0",
        "transformers>=4.36.0",
        "sentence-transformers",
        "tokenizers",
        "trl>=0.7.0",
        "datasets>=2.14.0",
        "gradio>=4.0.0,<5.0.0",
        "accelerate>=0.24.0",
        "safetensors>=0.4.0",
        "tqdm",
        "aiohttp",
        "aiofiles",
        "structlog",
        "textual>=0.19.0",
        "rich>=13.0.0",
        "ollama>=0.2.0",
        "requests>=2.31.0",
        "watchdog",
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
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: System :: Shells",
        "Topic :: Utilities",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
    entry_points={
        "console_scripts": [
            "todozi=todozi.tui:main",
            "tdz=tdz:main",
        ],
    },
)
