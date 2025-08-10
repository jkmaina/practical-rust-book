#!/usr/bin/env node
/**
 * Create simple game assets for the Bevy game example using Node.js Canvas.
 * This script creates a simple player sprite as a PNG file.
 */

const fs = require('fs');
const path = require('path');

// Simple canvas implementation for creating basic sprites
function createSimpleSprite() {
    // Create a simple SVG that can be converted to PNG
    const svgContent = `
<svg width="64" height="64" xmlns="http://www.w3.org/2000/svg">
  <!-- Blue square with white border -->
  <rect x="4" y="4" width="56" height="56" fill="#0064ff" stroke="#ffffff" stroke-width="2"/>
  
  <!-- Simple face -->
  <!-- Eyes -->
  <circle cx="20" cy="22" r="3" fill="#ffffff"/>
  <circle cx="44" cy="22" r="3" fill="#ffffff"/>
  
  <!-- Smile -->
  <path d="M 20 35 Q 32 45 44 35" stroke="#ffffff" stroke-width="2" fill="none"/>
</svg>`;
    
    return svgContent;
}

function createBackground() {
    const svgContent = `
<svg width="128" height="128" xmlns="http://www.w3.org/2000/svg">
  <!-- Green background -->
  <rect width="128" height="128" fill="#329632"/>
  
  <!-- Grass pattern -->
  <defs>
    <pattern id="grass" x="0" y="0" width="16" height="16" patternUnits="userSpaceOnUse">
      <rect width="16" height="16" fill="#329632"/>
      <rect x="0" y="0" width="8" height="8" fill="#287828"/>
    </pattern>
  </defs>
  <rect width="128" height="128" fill="url(#grass)"/>
</svg>`;
    
    return svgContent;
}

function main() {
    // Create assets directory if it doesn't exist
    const assetsDir = 'assets';
    if (!fs.existsSync(assetsDir)) {
        fs.mkdirSync(assetsDir);
    }
    
    console.log('🎨 Creating game assets...');
    
    // Create player sprite (as SVG for now)
    const playerSvg = createSimpleSprite();
    const playerPath = path.join(assetsDir, 'player.svg');
    fs.writeFileSync(playerPath, playerSvg);
    console.log(`✅ Created player sprite: ${playerPath}`);
    
    // Create background
    const bgSvg = createBackground();
    const bgPath = path.join(assetsDir, 'background.svg');
    fs.writeFileSync(bgPath, bgSvg);
    console.log(`✅ Created background: ${bgPath}`);
    
    // Create a simple fallback PNG using text representation
    const fallbackInstructions = `
# Bevy Game Assets

Since we created SVG files, you have a few options:

## Option 1: Convert SVG to PNG
Use an online converter or tool like Inkscape:
- Convert player.svg to player.png
- Convert background.svg to background.png

## Option 2: Use any 64x64 PNG image
- Download a simple sprite from the internet
- Create one in any image editor
- Name it 'player.png' and put it in the assets/ folder

## Option 3: Use the built-in Bevy shapes
The game will still run, it just won't show the sprite if the PNG is missing.

Run the game with: cargo run
`;
    
    fs.writeFileSync(path.join(assetsDir, 'README.txt'), fallbackInstructions);
    
    console.log('\n🎮 Assets created successfully!');
    console.log('Note: SVG files created. Convert to PNG or use any 64x64 image as player.png');
    console.log('You can now run the game with: cargo run');
}

if (require.main === module) {
    main();
}