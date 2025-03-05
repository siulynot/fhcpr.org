#!/bin/bash

# Check required directories
echo "Checking project structure..."

if [ ! -d "static" ]; then
  echo "ERROR: static directory not found!"
  echo "Please create the static directory at the project root."
  exit 1
else
  echo "✓ static directory exists"
fi

if [ ! -d "static/images" ]; then
  echo "ERROR: static/images directory not found!"
  echo "Please create the images directory inside the static directory."
  exit 1
else
  echo "✓ static/images directory exists"
fi

# Check for CSS file
if [ ! -f "static/styles.css" ]; then
  echo "ERROR: static/styles.css not found!"
  echo "Please make sure your CSS file is at static/styles.css"
  exit 1
else
  echo "✓ static/styles.css exists"
fi

# Check for required images
required_images=(
  "logo.png"
  "image4.png"
  "image5.png"
  "image6.png"
  "mmm.png"
  "mcs.png"
  "ssbv.png"
  "medicare.png"
  "humana.png"
  "pmc.png"
  "prossam.png"
)

missing_images=0
for img in "${required_images[@]}"; do
  if [ ! -f "static/images/$img" ]; then
    echo "WARNING: static/images/$img not found!"
    missing_images=$((missing_images+1))
  fi
done

if [ $missing_images -eq 0 ]; then
  echo "✓ All required images exist"
else
  echo "WARNING: $missing_images required images are missing. Please add them to static/images/"
fi

echo "Structure check complete."
