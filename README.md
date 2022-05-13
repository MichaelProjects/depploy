# Depploy

![example workflow](https://github.com/MichaelProjects/depploy/actions/workflows/depploy.yml/badge.svg?branch=master)
## About
Depploy is a cli tool that makes it easy to create docker images, for every programming language.
Simply specify the name and the version in your application config, and depploy will take care of build and pushing to the docker hub or your private registry.

## Installation
```bash
cargo install depploy
```

## Usage
Depploy is pretty simple, type:
```bash
depploy run
```
and depploy searches in the current directory for a (any file extionsion like yml, toml, yaml, ini):
- conf
- config
- Cargo

the config needs to contain following keys:

- name
- version


more configuration file are coming.

## Specifying Private Registry:
Depploy pushes without any configuration to the docker hub,
if you want to change this then you need to create following configuration file:

```bash
nano /etc/depploy/settings.toml
```
the content should be look like the "example_settings.toml".

## Todo
these are the features that should be added in the near future.

- Improved status and exit messages
- Better docker deamon handling
- Logging
- Dockerfile generator