let pyodide = null;
let ffmpeg = null;
let isDownloading = false;

// Rate limiting using localStorage
const RATE_LIMIT_KEY = 'ytdwt_downloads';
const DAILY_LIMIT = 10;

class RateLimiter {
    static getTodayKey() {
        const today = new Date().toISOString().split('T')[0];
        return `${RATE_LIMIT_KEY}_${today}`;
    }

    static getDownloadsToday() {
        const key = this.getTodayKey();
        return parseInt(localStorage.getItem(key) || '0');
    }

    static incrementDownloads() {
        const key = this.getTodayKey();
        const current = this.getDownloadsToday();
        localStorage.setItem(key, (current + 1).toString());
        return current + 1;
    }

    static canDownload() {
        return this.getDownloadsToday() < DAILY_LIMIT;
    }

    static getRemainingDownloads() {
        return DAILY_LIMIT - this.getDownloadsToday();
    }
}

// Update rate limit display
function updateRateLimitDisplay() {
    const statusEl = document.getElementById('rateLimitStatus');
    const downloadBtn = document.getElementById('downloadBtn');
    const remaining = RateLimiter.getRemainingDownloads();
    
    if (remaining > 0) {
        statusEl.textContent = `${remaining} downloads remaining today`;
        statusEl.className = 'mt-2 text-sm text-green-400';
        downloadBtn.disabled = false;
    } else {
        statusEl.textContent = 'Daily limit reached (10 downloads)';
        statusEl.className = 'mt-2 text-sm text-red-400';
        downloadBtn.disabled = true;
    }
}

// Initialize Pyodide
async function initializePyodide() {
    try {
        pyodide = await loadPyodide();
        await pyodide.loadPackage(['micropip']);
        
        // Install yt-dlp
        await pyodide.runPythonAsync(`
            import micropip
            await micropip.install('yt-dlp')
            import yt_dlp
            import json
            import sys
            from io import StringIO
        `);
        
        console.log('Pyodide and yt-dlp loaded successfully');
        return true;
    } catch (error) {
        console.error('Failed to initialize Pyodide:', error);
        showError('Failed to initialize YouTube downloader. Please refresh the page.');
        return false;
    }
}

// Initialize FFmpeg
async function initializeFFmpeg() {
    try {
        ffmpeg = new FFmpeg();
        ffmpeg.on('log', ({ message }) => {
            console.log('FFmpeg log:', message);
        });
        
        ffmpeg.on('progress', ({ progress }) => {
            updateProgress(progress * 100, 'Processing video...');
        });
        
        await ffmpeg.load();
        console.log('FFmpeg loaded successfully');
        return true;
    } catch (error) {
        console.error('Failed to initialize FFmpeg:', error);
        return false;
    }
}

// Start download process
async function startDownload() {
    if (isDownloading) return;
    
    const url = document.getElementById('urlInput').value.trim();
    if (!url) {
        showError('Please enter a YouTube URL');
        return;
    }

    // Check rate limit
    if (!RateLimiter.canDownload()) {
        showError('Daily download limit reached. Try again tomorrow.');
        return;
    }

    isDownloading = true;
    hideAllMessages();
    showProgress();
    setDownloadButtonState(true);

    try {
        // Initialize if not already done
        if (!pyodide) {
            updateProgress(10, 'Initializing downloader...');
            const success = await initializePyodide();
            if (!success) throw new Error('Failed to initialize Pyodide');
        }

        // Get download options
        const quality = document.getElementById('qualitySelect').value;
        const format = document.getElementById('formatSelect').value;
        const isPlaylist = document.getElementById('playlistCheckbox').checked;

        updateProgress(20, 'Fetching video info...');
        
        // Download the video
        const result = await downloadVideo(url, quality, format, isPlaylist);
        
        if (result.success) {
            // Increment download count
            RateLimiter.incrementDownloads();
            updateRateLimitDisplay();
            
            showSuccess(`Download completed! File saved as: ${result.filename}`);
        } else {
            showError(result.error);
        }
    } catch (error) {
        console.error('Download error:', error);
        showError(`Download failed: ${error.message}`);
    } finally {
        isDownloading = false;
        hideProgress();
        setDownloadButtonState(false);
    }
}

// Download video using yt-dlp
async function downloadVideo(url, quality, format, isPlaylist) {
    try {
        // Build yt-dlp options
        const options = {
            'quiet': False,
            'no_warnings': True,
            'progress_hooks': [progress_hook],
        };

        // Set output template
        options['outtmpl'] = '%(title)s.%(ext)s';

        // Format selection
        if (format === 'audio') {
            options['format'] = 'bestaudio';
            options['postprocessors'] = [{
                'key': 'FFmpegExtractAudio',
                'preferredcodec': 'mp3',
                'preferredquality': '192',
            }];
        } else {
            switch (quality) {
                case 'best':
                    options['format'] = 'bestvideo+bestaudio/best';
                    break;
                case '1080p':
                    options['format'] = 'bestvideo[height<=1080]+bestaudio/best[height<=1080]';
                    break;
                case '720p':
                    options['format'] = 'bestvideo[height<=720]+bestaudio/best[height<=720]';
                    break;
                case '480p':
                    options['format'] = 'bestvideo[height<=480]+bestaudio/best[height<=480]';
                    break;
            }
        }

        // Playlist handling
        if (!isPlaylist) {
            options['noplaylist'] = True;
        }

        // Run yt-dlp
        const pythonCode = `
            import yt_dlp
            import json
            
            options = ${JSON.stringify(options)}
            url = "${url}"
            
            try:
                with yt_dlp.YoutubeDL(options) as ydl:
                    info = ydl.extract_info(url, download=True)
                    filename = ydl.prepare_filename(info)
                    
                    # Return the filename
                    result = {
                        'success': True,
                        'filename': filename.split('/')[-1] if filename else 'downloaded_file'
                    }
            except Exception as e:
                result = {
                    'success': False,
                    'error': str(e)
                }
            
            result
        `;

        const result = await pyodide.runPythonAsync(pythonCode);
        
        // Convert Python result to JavaScript
        const jsResult = result.toJs();
        return {
            success: jsResult.get('success'),
            filename: jsResult.get('filename'),
            error: jsResult.get('error')
        };

    } catch (error) {
        return {
            success: false,
            error: error.message
        };
    }
}

// Progress hook for yt-dlp
function progress_hook(d) {
    if (d['status'] === 'downloading') {
        const percent = d['_percent_str'] || '0.0%';
        const progress = parseFloat(percent.replace('%', ''));
        updateProgress(progress, d['status'] || 'Downloading...');
    } else if (d['status'] === 'finished') {
        updateProgress(100, 'Processing completed');
    }
}

// UI Helper Functions
function updateProgress(percent, status) {
    document.getElementById('progressBar').style.width = `${percent}%`;
    document.getElementById('progressText').textContent = `${Math.round(percent)}%`;
    document.getElementById('statusText').textContent = status;
}

function showProgress() {
    document.getElementById('progressSection').style.display = 'block';
}

function hideProgress() {
    document.getElementById('progressSection').style.display = 'none';
}

function showError(message) {
    document.getElementById('errorText').textContent = message;
    document.getElementById('errorSection').style.display = 'block';
}

function hideError() {
    document.getElementById('errorSection').style.display = 'none';
}

function showSuccess(message) {
    document.getElementById('successText').textContent = message;
    document.getElementById('successSection').style.display = 'block';
}

function hideSuccess() {
    document.getElementById('successSection').style.display = 'none';
}

function hideAllMessages() {
    hideError();
    hideSuccess();
}

function setDownloadButtonState(disabled) {
    const btn = document.getElementById('downloadBtn');
    btn.disabled = disabled;
    if (disabled) {
        btn.textContent = 'Downloading...';
        btn.classList.add('opacity-75');
    } else {
        btn.textContent = 'Download';
        btn.classList.remove('opacity-75');
    }
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', async () => {
    updateRateLimitDisplay();
    
    // Pre-initialize components in background
    setTimeout(async () => {
        await initializePyodide();
        await initializeFFmpeg();
    }, 1000);
});

// Handle Enter key in URL input
document.getElementById('urlInput').addEventListener('keypress', (e) => {
    if (e.key === 'Enter' && !isDownloading) {
        startDownload();
    }
});
