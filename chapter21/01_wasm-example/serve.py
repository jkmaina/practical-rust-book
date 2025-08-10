#!/usr/bin/env python3
"""
Simple HTTP server for serving WASM files with correct MIME types.
Run this script and open http://localhost:8000 in your browser.
"""

import http.server
import socketserver
import os
import sys

class WASMHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add WASM MIME type
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        # Add CORS headers for local development
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

def main():
    port = 8000
    
    # Change to the directory containing this script
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    print(f"🚀 Starting WASM development server...")
    print(f"📁 Serving files from: {os.getcwd()}")
    print(f"🌐 Open your browser to: http://localhost:{port}")
    print(f"🛑 Press Ctrl+C to stop the server")
    print()
    
    try:
        with socketserver.TCPServer(("", port), WASMHandler) as httpd:
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Server stopped")
        sys.exit(0)
    except OSError as e:
        if e.errno == 10048:  # Port already in use on Windows
            print(f"❌ Port {port} is already in use. Try a different port or stop the other server.")
        else:
            print(f"❌ Error starting server: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()