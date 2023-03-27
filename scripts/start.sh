#!/bin/bash
set -e # Exit with nonzero exit code if anything fails
# Set working directory to frontend/
cd frontend

# Install dependencies
npm install

# Build the project
npm run build

# Set working directory to ../
cd ..

bin/kalkafox-img
