# Depploy

## about
Depploy is a cli tool that makes it easy to create docker images, for every programming language.
Simply specify the name and the version in your application config, and depploy will take care of build and pushing to the docker hub or your private registry.

## Usage
Depploy is pretty simple, type:
```bash
depploy run
```
and depploy searches in the current directory for a:
- conf.toml
- Cargo.toml

more configuration file are coming.

## Specifying Private Registry:
Depploy pushes without any configuration to the docker hub,
if you want to change this then you need to create following configuration file:

```bash
mkdir /etc/depploy
nano /etc/depploy/settings.toml
```
the content should be look like the "example_settings.toml".