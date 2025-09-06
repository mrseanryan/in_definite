#!/bin/bash
set -e
echo "Testing published package..."
./go-published.sh umbrella
echo "Test [done]"
