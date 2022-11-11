# Depploy

![example workflow](https://github.com/MichaelProjects/depploy/actions/workflows/depploy.yml/badge.svg?branch=master)
## About
Depploy is a cli tool that makes it easy to create docker images, for every programming language.
Simply specify the name and the version in your application config, and depploy will take care of build and pushing to the docker hub or your private registry.

## Installation
```bash
cargo install depploy
```

## Usage (command: Run)
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

## Usage (command: generate)
This command uses the specified path and index all file extionsion with that are not ignore via the ".gitignore". If the programming language is currently supported by depploy, it will create a dockerfile in the root of your folder.

```bash
depploy generate
```
### Language not supported?
You are welcome to submit a pull request to the [dockerfile](https://github.com/MichaelProjects/depploy/tree/dockerfiles) branch to add your basic instruction file to depploy.

## Specifying Private Registry:
Depploy pushes without any configuration to the docker hub,
if you want to change this then you need to create following configuration file:

```bash
nano "~/.depploy/settings.toml
```
the content should be look like the "example_settings.toml".

## Todo
these are the features that should be added in the near future.

- Improved status and exit messages
- Better docker deamon handling
- Generate subcommands like list files, add files (custom or create pull request).

## Changelog

## 0.3.1
- Fixed issues and created release

## 0.3.0
- Improved error handling in image build process.
- Added prototype service (backend service comming soon..)
- Refactored codebase structure

### 0.2.0 
- Added generate command.
- Improved error handling for run command.
- moved depploy dir into home directory of user.

### 0.1.0 
- inital release with run command.