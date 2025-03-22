/**
 * LightNVR Web Interface Video Recordings
 * Contains functionality for recording playback
 */

/**
 * Play recording - Enhanced implementation with animations
 */
function playRecording(recordingId) {
    const videoModal = document.getElementById('video-modal');
    const modalContent = videoModal?.querySelector('.modal-content');
    const videoPlayer = document.getElementById('video-player');
    const videoTitle = document.getElementById('video-modal-title');
    const videoCloseBtn = document.getElementById('video-close-btn');
    const modalCloseBtn = videoModal?.querySelector('.close');
    const loadingIndicator = videoPlayer?.querySelector('.loading-indicator');

    if (!videoModal || !videoPlayer || !videoTitle || !modalContent) {
        console.error('Video modal elements not found');
        return;
    }

    // Define close modal function
    function closeModal() {
        // Add closing animations
        modalContent.style.opacity = '0';
        modalContent.style.transform = 'scale(0.95)';
        videoModal.style.opacity = '0';
        
        // Wait for animation to complete before hiding
        setTimeout(() => {
            videoModal.classList.add('hidden');
            videoModal.style.display = 'none';
            
            // Stop video playback
            const videoElement = videoPlayer.querySelector('video');
            if (videoElement) {
                videoElement.pause();
                videoElement.src = '';
            }
        }, 300);
    }

    // Set up close button event handlers
    if (videoCloseBtn) {
        videoCloseBtn.onclick = closeModal;
    }
    
    if (modalCloseBtn) {
        modalCloseBtn.onclick = closeModal;
    }

    // Close on click outside
    window.onclick = function(event) {
        if (event.target === videoModal) {
            closeModal();
        }
    };

    // Show loading state and make modal visible
    if (loadingIndicator) {
        loadingIndicator.style.display = 'flex';
    }
    
    // Reset modal content for animation
    modalContent.style.opacity = '0';
    modalContent.style.transform = 'scale(0.95)';
    
    // Show modal with fade-in
    videoModal.classList.remove('hidden');
    videoModal.style.display = 'flex';
    videoModal.style.opacity = '0';
    
    // Trigger animations after a small delay to ensure display change is processed
    setTimeout(() => {
        videoModal.style.opacity = '1';
        modalContent.style.opacity = '1';
        modalContent.style.transform = 'scale(1)';
    }, 10);

    // Fetch recording details
    fetch(`/api/recordings/${recordingId}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load recording details');
            }
            return response.json();
        })
        .then(recording => {
            console.log('Recording details:', recording);

            // Set video title
            videoTitle.textContent = `${recording.stream} - ${recording.start_time}`;

            // Base URLs for video
            const videoUrl = `/api/recordings/play/${recordingId}`;
            const downloadUrl = `/api/recordings/download/${recordingId}?download=1`;

            console.log('Video URL:', videoUrl);
            console.log('Download URL:', downloadUrl);

            // Reset video player with loading indicator
            videoPlayer.innerHTML = `
                <video class="w-full h-full object-contain" controls></video>
                <div class="loading-indicator absolute inset-0 flex flex-col items-center justify-center bg-black bg-opacity-70">
                    <div class="loading-spinner mb-4"></div>
                    <span class="text-white">Loading video...</span>
                </div>
            `;
            const videoElement = videoPlayer.querySelector('video');

            // Determine if HLS or MP4
            if (recording.path && recording.path.endsWith('.m3u8')) {
                if (Hls && Hls.isSupported()) {
                    let hls = new Hls();
                    hls.loadSource(videoUrl);
                    hls.attachMedia(videoElement);
                    hls.on(Hls.Events.MANIFEST_PARSED, function () {
                        videoElement.play();
                    });
                } else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
                    videoElement.src = videoUrl;
                    videoElement.play();
                } else {
                    console.error('HLS not supported');
                    videoPlayer.innerHTML = `
                        <div style="height:70vh;display:flex;flex-direction:column;justify-content:center;align-items:center;background:#000;color:#fff;padding:20px;text-align:center;">
                            <p style="font-size:18px;margin-bottom:10px;">HLS playback is not supported in this browser.</p>
                            <a href="${downloadUrl}" class="btn btn-primary" download>Download Video</a>
                        </div>
                    `;
                }
            } else {
                // Standard MP4 playback
                videoElement.src = videoUrl;
                videoElement.play().catch(e => console.error('Error playing video:', e));
            }

            // Update download button
            const downloadBtn = document.getElementById('video-download-btn');
            if (downloadBtn) {
                downloadBtn.onclick = function(e) {
                    e.preventDefault();
                    const link = document.createElement('a');
                    link.href = downloadUrl;
                    link.download = '';
                    document.body.appendChild(link);
                    link.click();
                    document.body.removeChild(link);
                    return false;
                };
            }

            // Add event listener to hide loading indicator when video is ready
            videoElement.addEventListener('loadeddata', function() {
                const updatedLoadingIndicator = videoPlayer.querySelector('.loading-indicator');
                if (updatedLoadingIndicator) {
                    updatedLoadingIndicator.style.display = 'none';
                }
            });
        })
        .catch(error => {
            console.error('Error loading recording:', error);
            // Hide loading indicator when video fails to load
            if (loadingIndicator) {
                loadingIndicator.style.display = 'none';
            }
            videoPlayer.innerHTML = `
                <div style="height:70vh;display:flex;flex-direction:column;justify-content:center;align-items:center;background:#000;color:#fff;padding:20px;text-align:center;">
                    <p style="font-size:18px;margin-bottom:10px;">Error: ${error.message}</p>
                    <p style="margin-bottom:20px;">Cannot load the recording.</p>
                    <a href="/api/recordings/play/${recordingId}" class="btn btn-primary">Play Video</a>
                    <a href="/api/recordings/download/${recordingId}?download=1" class="btn btn-primary" download>Download Video</a>
                </div>
            `;
        });
}

// Export functions
window.playRecording = playRecording;
