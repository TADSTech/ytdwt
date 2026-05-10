# YT-DWT Web - Serverless YouTube Downloader

A browser-based YouTube downloader that runs entirely in the client using WebAssembly. No server required - all processing happens locally in your browser.

## Features

- **Serverless Architecture**: Runs completely in the browser using Pyodide and WebAssembly
- **Rate Limiting**: 10 downloads per day per browser (stored in localStorage)
- **Multiple Formats**: Download video or extract audio as MP3
- **Quality Options**: Best available, 1080p, 720p, 480p
- **Playlist Support**: Download individual videos or entire playlists
- **Progress Tracking**: Real-time download progress
- **Modern UI**: Clean, responsive interface with Tailwind CSS

## Technology Stack

- **Pyodide**: Python runtime in WebAssembly
- **yt-dlp**: YouTube downloader running in Pyodide
- **FFmpeg.wasm**: Video processing in the browser
- **Tailwind CSS**: Modern UI styling
- **Vanilla JavaScript**: No framework dependencies

## Rate Limiting

Since this is a serverless application, rate limiting is implemented using localStorage:

- **10 downloads per day per browser**
- Resets automatically at midnight (local timezone)
- Persists across browser sessions
- Cannot be bypassed without clearing browser data

## Security & Privacy

- **100% Client-Side**: No files uploaded to any server
- **Local Processing**: All downloads and processing happen in your browser
- **No Tracking**: No analytics or tracking scripts
- **Open Source**: Full transparency of code

## Usage

1. Open `index.html` in a modern web browser
2. Paste a YouTube URL
3. Select quality and format options
4. Click "Download"
5. Files are saved to your browser's default download location

## Browser Support

- Chrome 67+
- Firefox 79+
- Safari 15+
- Edge 79+

## Limitations

- **Browser Memory**: Large files may be limited by available browser memory
- **Processing Speed**: Slower than native desktop applications
- **YouTube Changes**: May break when YouTube updates their systems
- **File Size**: Some browsers limit download sizes

## Development

### Local Testing

1. Start a local HTTP server (required for CORS):
```bash
# Python 3
python -m http.server 8000

# Node.js
npx serve .

# Or use any static file server
```

2. Open `http://localhost:8000` in your browser

### File Structure

```
web/
├── index.html          # Main application
├── app.js             # Core functionality
├── README.md          # This file
└── ../app_icon.png    # Application icon
```

## Troubleshooting

### "Failed to initialize Pyodide"
- Check browser compatibility
- Ensure you're using HTTPS or localhost
- Try refreshing the page

### "Download failed"
- Verify the YouTube URL is correct
- Check if the video is region-restricted
- Try again (YouTube may have updated their API)

### "Rate limit reached"
- Wait until tomorrow (resets at midnight)
- Or clear browser data (not recommended)

## Contributing

This is a proof-of-concept implementation. Contributions welcome for:

- Error handling improvements
- UI/UX enhancements
- Performance optimizations
- Additional format support

## License

MIT License - see parent directory for details.

## Acknowledgments

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - YouTube downloader
- [Pyodide](https://pyodide.org/) - Python in WebAssembly
- [FFmpeg.wasm](https://ffmpegwasm.netlify.app/) - Video processing in browser
- [Tailwind CSS](https://tailwindcss.com/) - CSS framework
