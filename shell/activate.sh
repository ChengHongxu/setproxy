#!/bin/sh
# Conda activation script for setproxy
# This defines the setproxy function when the environment is activated

setproxy() {
    eval "$(_setproxy "$@")"
}
