let sessionId = null;
let ws = null;

const uploadArea = document.getElementById('upload-area');
const fileInput = document.getElementById('file-input');
const fileInfo = document.getElementById('file-info');
const filename = document.getElementById('filename');
const progressCard = document.getElementById('progress-card');
const progressBar = document.getElementById('progress-bar');
const progressPercent = document.getElementById('progress-percent');
const progressText = document.getElementById('progress-text');
const statusMessage = document.getElementById('status-message');
const downloadBtn = document.getElementById('download-btn');

// Click to upload
uploadArea.addEventListener('click', () => fileInput.click());

// Drag and drop
uploadArea.addEventListener('dragover', (e) => {
    e.preventDefault();
    uploadArea.classList.add('border-indigo-500', 'bg-indigo-50');
});

uploadArea.addEventListener('dragleave', () => {
    uploadArea.classList.remove('border-indigo-500', 'bg-indigo-50');
});

uploadArea.addEventListener('drop', (e) => {
    e.preventDefault();
    uploadArea.classList.remove('border-indigo-500', 'bg-indigo-50');
    
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        handleFile(files[0]);
    }
});

// File input change
fileInput.addEventListener('change', (e) => {
    if (e.target.files.length > 0) {
        handleFile(e.target.files[0]);
    }
});

async function handleFile(file) {
    if (!file.name.endsWith('.mp4')) {
        alert('Please select an MP4 file');
        return;
    }

    filename.textContent = file.name;
    fileInfo.classList.remove('hidden');
    progressCard.classList.remove('hidden');
    downloadBtn.classList.add('hidden');

    const formData = new FormData();
    formData.append('file', file);

    try {
        const response = await fetch('/api/upload', {
            method: 'POST',
            body: formData
        });

        const data = await response.json();
        
        if (response.ok) {
            sessionId = data.session_id;
            connectWebSocket(sessionId);
        } else {
            alert(data.error || 'Upload failed');
        }
    } catch (error) {
        alert('Upload failed: ' + error.message);
    }
}

function connectWebSocket(sessionId) {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${protocol}//${window.location.host}/api/ws/${sessionId}`);

    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        updateProgress(data.progress, data.message, data.status);
    };

    ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        statusMessage.textContent = 'Connection error occurred';
    };

    ws.onclose = () => {
        console.log('WebSocket closed');
    };
}

function updateProgress(progress, message, status) {
    progressBar.style.width = `${progress}%`;
    progressPercent.textContent = `${Math.round(progress)}%`;
    progressText.textContent = message;
    statusMessage.textContent = status === 'completed' ? '✅ Conversion completed!' : '⚙️ Processing...';

    if (status === 'completed') {
        downloadBtn.classList.remove('hidden');
        downloadBtn.onclick = () => {
            window.location.href = `/api/download/${sessionId}`;
        };
    }
}