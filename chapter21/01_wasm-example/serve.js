#!/usr/bin/env node
/**
 * Simple Node.js HTTP server for serving WASM files with correct MIME types.
 * Run this script and open http://localhost:8000 in your browser.
 */

const http = require('http');
const fs = require('fs');
const path = require('path');
const url = require('url');

// MIME types
const mimeTypes = {
    '.html': 'text/html',
    '.js': 'text/javascript',
    '.css': 'text/css',
    '.json': 'application/json',
    '.png': 'image/png',
    '.jpg': 'image/jpg',
    '.gif': 'image/gif',
    '.svg': 'image/svg+xml',
    '.wav': 'audio/wav',
    '.mp4': 'video/mp4',
    '.woff': 'application/font-woff',
    '.ttf': 'application/font-ttf',
    '.eot': 'application/vnd.ms-fontobject',
    '.otf': 'application/font-otf',
    '.wasm': 'application/wasm'
};

const server = http.createServer((req, res) => {
    const parsedUrl = url.parse(req.url);
    let pathname = `.${parsedUrl.pathname}`;
    
    // Default to index.html
    if (pathname === './') {
        pathname = './index.html';
    }
    
    const ext = path.parse(pathname).ext;
    const mimeType = mimeTypes[ext] || 'text/plain';
    
    fs.readFile(pathname, (err, data) => {
        if (err) {
            res.statusCode = 404;
            res.setHeader('Content-Type', 'text/plain');
            res.end('File not found');
            return;
        }
        
        // Set CORS headers for local development
        res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
        res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
        res.setHeader('Content-Type', mimeType);
        
        res.statusCode = 200;
        res.end(data);
    });
});

const port = 8000;

server.listen(port, () => {
    console.log('🚀 Starting WASM development server...');
    console.log(`📁 Serving files from: ${process.cwd()}`);
    console.log(`🌐 Open your browser to: http://localhost:${port}`);
    console.log('🛑 Press Ctrl+C to stop the server');
    console.log();
});

server.on('error', (err) => {
    if (err.code === 'EADDRINUSE') {
        console.log(`❌ Port ${port} is already in use. Try a different port or stop the other server.`);
    } else {
        console.log(`❌ Error starting server: ${err.message}`);
    }
    process.exit(1);
});

process.on('SIGINT', () => {
    console.log('\n🛑 Server stopped');
    process.exit(0);
});