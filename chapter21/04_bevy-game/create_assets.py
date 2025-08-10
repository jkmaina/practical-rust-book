#!/usr/bin/env python3
"""
Create simple game assets for the Bevy game example.
This script creates a simple player sprite as a PNG file.
"""

from PIL import Image, ImageDraw
import os

def create_player_sprite():
    """Create a simple player sprite (blue square with white border)"""
    size = 64
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))  # Transparent background
    draw = ImageDraw.Draw(img)
    
    # Draw blue square with white border
    draw.rectangle([4, 4, size-4, size-4], fill=(0, 100, 255, 255), outline=(255, 255, 255, 255), width=2)
    
    # Add a simple face
    # Eyes
    draw.ellipse([16, 20, 20, 24], fill=(255, 255, 255, 255))
    draw.ellipse([44, 20, 48, 24], fill=(255, 255, 255, 255))
    
    # Smile
    draw.arc([20, 35, 44, 50], start=0, end=180, fill=(255, 255, 255, 255), width=2)
    
    return img

def create_background():
    """Create a simple background texture"""
    size = 128
    img = Image.new('RGB', (size, size), (50, 150, 50))  # Green background
    draw = ImageDraw.Draw(img)
    
    # Add some grass-like texture
    for i in range(0, size, 16):
        for j in range(0, size, 16):
            if (i + j) % 32 == 0:
                draw.rectangle([i, j, i+8, j+8], fill=(40, 120, 40))
    
    return img

def main():
    # Create assets directory if it doesn't exist
    assets_dir = 'assets'
    if not os.path.exists(assets_dir):
        os.makedirs(assets_dir)
    
    print("🎨 Creating game assets...")
    
    # Create player sprite
    player_img = create_player_sprite()
    player_path = os.path.join(assets_dir, 'player.png')
    player_img.save(player_path)
    print(f"✅ Created player sprite: {player_path}")
    
    # Create background
    bg_img = create_background()
    bg_path = os.path.join(assets_dir, 'background.png')
    bg_img.save(bg_path)
    print(f"✅ Created background: {bg_path}")
    
    print("\n🎮 Assets created successfully!")
    print("You can now run the game with: cargo run")

if __name__ == "__main__":
    try:
        main()
    except ImportError:
        print("❌ PIL (Pillow) is required to create assets.")
        print("Install it with: pip install Pillow")
        print("\nAlternatively, you can:")
        print("1. Create a 64x64 PNG file named 'player.png' in the assets/ folder")
        print("2. Use any image editing software or download a sprite online")
        print("3. Run: cargo run")