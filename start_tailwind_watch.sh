#!/bin/bash

# If not installed, run:
# npm install tailwindcss @tailwindcss/cli

npx @tailwindcss/cli -i ./assets/css/tailwind_input.css -o ./assets/css/tailwind_output.css --watch
