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
                <div class="w-full max-w-full">
                    <video class="w-full h-auto max-w-full object-contain" controls></video>
                    <div class="loading-indicator absolute inset-0 flex flex-col items-center justify-center bg-black bg-opacity-70">
                        <div class="loading-spinner mb-4"></div>
                        <span class="text-white">Loading video...</span>
                    </div>
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

            // Create controls container with responsive Tailwind classes
            const controlsContainer = document.createElement('div');
            controlsContainer.id = 'recordings-controls';
            controlsContainer.className = 'mx-4 mb-4 p-4 border border-green-500 rounded-lg bg-white dark:bg-gray-700 shadow-md relative z-10';
            
            // Create heading
            const heading = document.createElement('h3');
            heading.className = 'text-lg font-bold text-center mb-4 text-gray-800 dark:text-white';
            heading.textContent = 'PLAYBACK CONTROLS';
            controlsContainer.appendChild(heading);
            
            // Create controls grid container
            const controlsGrid = document.createElement('div');
            controlsGrid.className = 'grid grid-cols-1 md:grid-cols-2 gap-4 mb-2';
            controlsContainer.appendChild(controlsGrid);
            
            // Create playback section
            const playbackSection = document.createElement('div');
            playbackSection.className = 'border-b pb-4 md:border-b-0 md:border-r md:pr-4 md:pb-0';
            
            // Create playback section heading
            const playbackHeading = document.createElement('h4');
            playbackHeading.className = 'font-bold text-center mb-3 text-gray-700 dark:text-gray-300';
            playbackHeading.textContent = 'Playback Speed';
            playbackSection.appendChild(playbackHeading);
            
            // Create playback info
            const playbackInfo = document.createElement('div');
            playbackInfo.className = 'text-center text-sm text-gray-600 dark:text-gray-400 mt-4';
            playbackInfo.textContent = 'Use video player controls to adjust playback';
            playbackSection.appendChild(playbackInfo);
            
            // Add playback section to grid
            controlsGrid.appendChild(playbackSection);
            
            // Create detection section
            const detectionSection = document.createElement('div');
            detectionSection.className = 'pt-4 md:pt-0 md:pl-4';
            
            // Create detection section heading
            const detectionHeading = document.createElement('h4');
            detectionHeading.className = 'font-bold text-center mb-2 text-gray-700 dark:text-gray-300';
            detectionHeading.textContent = 'Detection Overlays';
            detectionSection.appendChild(detectionHeading);
            
            // Create detection status indicator
            const detectionStatusIndicator = document.createElement('div');
            detectionStatusIndicator.className = 'text-center text-sm text-gray-600 dark:text-gray-400 mt-4';
            detectionStatusIndicator.textContent = 'No detections found for this recording';
            detectionSection.appendChild(detectionStatusIndicator);
            
            // Add detection section to grid
            controlsGrid.appendChild(detectionSection);
            
            // Add download button to controls container
            const downloadSection = document.createElement('div');
            downloadSection.className = 'flex justify-center mt-4 pt-2 border-t border-gray-200 dark:border-gray-700';
            
            const downloadButton = document.createElement('a');
            downloadButton.className = 'px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center text-sm';
            downloadButton.innerHTML = `
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
                Download Video
            `;
            downloadButton.href = downloadUrl;
            downloadButton.download = '';
            
            downloadSection.appendChild(downloadButton);
            controlsContainer.appendChild(downloadSection);
            
            // Add controls to player
            videoPlayer.appendChild(controlsContainer);

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
