/**
 * LightNVR Web Interface UI Components
 * Preact components for UI elements like modals, status messages, etc.
 */

import { h } from '../../preact.min.js';
import { useState, useEffect, useRef, useCallback } from '../../preact.hooks.module.js';
import { html } from '../../preact-app.js';
import { statusMessageStore, snapshotModalStore, videoModalStore } from '../../preact-app.js';

/**
 * Status message component
 * @returns {JSX.Element} Status message component
 */
export function StatusMessage() {
  const [state, setState] = useState(statusMessageStore.getState());
  
  useEffect(() => {
    // Subscribe to store changes
    const unsubscribe = statusMessageStore.subscribe(newState => {
      setState(newState);
    });
    
    // Unsubscribe when component unmounts
    return unsubscribe;
  }, []);
  
  return html`
    <div 
      id="status-message"
      class=${`status-message ${state.visible ? 'visible' : ''}`}
    >
      ${state.message}
    </div>
  `;
}

/**
 * Modal component
 * @param {Object} props - Component props
 * @param {string} props.id - Modal ID
 * @param {string} props.title - Modal title
 * @param {boolean} props.isOpen - Whether the modal is open
 * @param {Function} props.onClose - Function to call when the modal is closed
 * @param {JSX.Element} props.children - Modal content
 * @param {JSX.Element} props.footer - Modal footer
 * @returns {JSX.Element} Modal component
 */
export function Modal({ id, title, isOpen, onClose, children, footer }) {
  const modalRef = useRef(null);
  
  // Close modal when clicking outside
  useEffect(() => {
    const handleOutsideClick = (e) => {
      if (modalRef.current && e.target === modalRef.current) {
        onClose();
      }
    };
    
    if (isOpen) {
      document.addEventListener('click', handleOutsideClick);
      document.body.style.overflow = 'hidden';
    }
    
    return () => {
      document.removeEventListener('click', handleOutsideClick);
      document.body.style.overflow = '';
    };
  }, [isOpen, onClose]);
  
  // Close modal when pressing Escape
  useEffect(() => {
    const handleEscape = (e) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };
    
    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
    }
    
    return () => {
      document.removeEventListener('keydown', handleEscape);
    };
  }, [isOpen, onClose]);
  
  if (!isOpen) return null;
  
  return html`
    <div 
      id=${id}
      ref=${modalRef}
      class="modal fixed inset-0 z-50 overflow-auto bg-black bg-opacity-70 backdrop-blur-sm flex items-center justify-center"
    >
      <div class="modal-content bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-3xl mx-4 overflow-hidden transform transition-transform duration-300 scale-100 opacity-100">
        <div class="modal-header flex justify-between items-center p-5 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-100">${title}</h3>
          <span 
            class="close text-2xl font-bold text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 cursor-pointer hover:scale-110 transition-transform"
            onClick=${onClose}
          >
            &times;
          </span>
        </div>
        <div class="modal-body p-6 bg-gray-50 dark:bg-gray-900">
          ${children}
        </div>
        ${footer && html`
          <div class="modal-footer flex justify-end gap-3 p-5 border-t border-gray-200 dark:border-gray-700">
            ${footer}
          </div>
        `}
      </div>
    </div>
  `;
}

/**
 * Snapshot preview modal component
 * @returns {JSX.Element} Snapshot preview modal component
 */
export function SnapshotPreviewModal() {
  const [state, setState] = useState(snapshotModalStore.getState());
  const [isOpen, setIsOpen] = useState(false);
  
  useEffect(() => {
    // Subscribe to store changes
    const unsubscribe = snapshotModalStore.subscribe(newState => {
      setState(newState);
      setIsOpen(!!newState.imageUrl);
    });
    
    // Unsubscribe when component unmounts
    return unsubscribe;
  }, []);
  
  const handleClose = useCallback(() => {
    setIsOpen(false);
    snapshotModalStore.setState({ imageUrl: '', title: '' });
  }, []);
  
  const handleDownload = useCallback(() => {
    if (!state.imageUrl) return;
    
    // Create download link
    const link = document.createElement('a');
    link.href = state.imageUrl;
    link.download = `snapshot_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    
    // Show success message
    statusMessageStore.setState({
      message: 'Download started',
      visible: true,
      timeout: setTimeout(() => {
        statusMessageStore.setState(state => ({ ...state, visible: false }));
      }, 3000)
    });
  }, [state.imageUrl]);
  
  const footer = html`
    <button 
      id="snapshot-download-btn" 
      class="btn-primary px-5 py-2.5 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 font-medium"
      onClick=${handleDownload}
    >
      <svg class="w-4 h-4 inline-block mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
      </svg>
      Download
    </button>
    <button 
      id="snapshot-close-btn" 
      class="btn-secondary px-5 py-2.5 bg-gray-600 text-white rounded-md hover:bg-gray-700 transition-colors shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-opacity-50 font-medium"
      onClick=${handleClose}
    >
      Close
    </button>
  `;
  
  return html`
    <${Modal}
      id="snapshot-preview-modal"
      title=${state.title || "Snapshot Preview"}
      isOpen=${isOpen}
      onClose=${handleClose}
      footer=${footer}
    >
      <div class="relative rounded-lg overflow-hidden shadow-inner bg-black">
        <img 
          id="snapshot-preview-image" 
          src=${state.imageUrl} 
          alt="Snapshot Preview" 
          class="w-full h-auto max-h-[70vh] object-contain mx-auto"
        />
      </div>
    </${Modal}>
  `;
}

/**
 * Video modal component
 * @returns {JSX.Element} Video modal component
 */
export function VideoModal() {
  const [state, setState] = useState(videoModalStore.getState());
  const [isOpen, setIsOpen] = useState(false);
  const videoRef = useRef(null);
  
  useEffect(() => {
    // Subscribe to store changes
    const unsubscribe = videoModalStore.subscribe(newState => {
      setState(newState);
      setIsOpen(!!newState.videoUrl);
    });
    
    // Unsubscribe when component unmounts
    return unsubscribe;
  }, []);
  
  // Set up video player when videoUrl changes
  useEffect(() => {
    if (!isOpen || !state.videoUrl || !videoRef.current) return;
    
    const video = videoRef.current;
    
    if (state.videoUrl.endsWith('.m3u8')) {
      // HLS video
      if (window.Hls && window.Hls.isSupported()) {
        const hls = new window.Hls();
        hls.loadSource(state.videoUrl);
        hls.attachMedia(video);
        hls.on(window.Hls.Events.MANIFEST_PARSED, () => {
          video.play().catch(e => console.error('Error auto-playing video:', e));
        });
        
        // Store hls instance for cleanup
        video.hlsInstance = hls;
      } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
        // Native HLS support
        video.src = state.videoUrl;
        video.play().catch(e => console.error('Error auto-playing video:', e));
      }
    } else {
      // Regular video
      video.src = state.videoUrl;
      video.play().catch(e => console.error('Error auto-playing video:', e));
    }
    
    return () => {
      // Cleanup
      if (video.hlsInstance) {
        video.hlsInstance.destroy();
        delete video.hlsInstance;
      }
      video.pause();
      video.src = '';
    };
  }, [isOpen, state.videoUrl]);
  
  const handleClose = useCallback(() => {
    setIsOpen(false);
    
    // Cleanup video
    if (videoRef.current) {
      if (videoRef.current.hlsInstance) {
        videoRef.current.hlsInstance.destroy();
        delete videoRef.current.hlsInstance;
      }
      videoRef.current.pause();
      videoRef.current.src = '';
    }
    
    videoModalStore.setState({ videoUrl: '', title: '', downloadUrl: '' });
  }, []);
  
  const handleDownload = useCallback(() => {
    if (!state.downloadUrl) return;
    
    // Create download link
    const link = document.createElement('a');
    link.href = state.downloadUrl;
    link.download = `recording_${new Date().toISOString().replace(/[:.]/g, '-')}.mp4`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    
    // Show success message
    statusMessageStore.setState({
      message: 'Download started',
      visible: true,
      timeout: setTimeout(() => {
        statusMessageStore.setState(state => ({ ...state, visible: false }));
      }, 3000)
    });
  }, [state.downloadUrl]);
  
  const footer = html`
    <button 
      id="video-download-btn" 
      class="btn-primary px-5 py-2.5 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 font-medium"
      onClick=${handleDownload}
    >
      <svg class="w-4 h-4 inline-block mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
      </svg>
      Download
    </button>
    <button 
      id="video-close-btn" 
      class="btn-secondary px-5 py-2.5 bg-gray-600 text-white rounded-md hover:bg-gray-700 transition-colors shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-opacity-50 font-medium"
      onClick=${handleClose}
    >
      Close
    </button>
  `;
  
  return html`
    <${Modal}
      id="video-modal"
      title=${state.title || "Video Playback"}
      isOpen=${isOpen}
      onClose=${handleClose}
      footer=${footer}
    >
      <div id="video-player" class="relative rounded-lg overflow-hidden shadow-inner bg-black">
        <video 
          ref=${videoRef}
          class="w-full h-auto max-h-[70vh] object-contain mx-auto"
          controls
          autoPlay
        ></video>
      </div>
    </${Modal}>
  `;
}

/**
 * Show status message
 * @param {string} message - Message to show
 * @param {number} duration - Duration in milliseconds
 */
export function showStatusMessage(message, duration = 3000) {
  const state = statusMessageStore.getState();
  
  // Clear any existing timeout
  if (state.timeout) {
    clearTimeout(state.timeout);
  }
  
  // Update state
  statusMessageStore.setState({
    message,
    visible: true,
    timeout: setTimeout(() => {
      statusMessageStore.setState(state => ({ ...state, visible: false }));
    }, duration)
  });
}

/**
 * Show snapshot preview
 * @param {string} imageUrl - Image URL
 * @param {string} title - Modal title
 */
export function showSnapshotPreview(imageUrl, title = 'Snapshot Preview') {
  snapshotModalStore.setState({
    imageUrl,
    title
  });
}

/**
 * Show video modal
 * @param {string} videoUrl - Video URL
 * @param {string} title - Modal title
 */
export function showVideoModal(videoUrl, title = 'Video Playback') {
  const downloadUrl = videoUrl.replace('/play/', '/download/');
  
  videoModalStore.setState({
    videoUrl,
    title,
    downloadUrl
  });
}

/**
 * Setup modals
 */
export function setupModals() {
  // Render UI components to the DOM
  import('../../preact.min.js').then(({ render }) => {
    // Status message
    const statusMessageContainer = document.getElementById('status-message-container');
    if (!statusMessageContainer) {
      const container = document.createElement('div');
      container.id = 'status-message-container';
      document.body.appendChild(container);
      render(html`<${StatusMessage} />`, container);
    }
    
    // Snapshot preview modal
    const snapshotModalContainer = document.getElementById('snapshot-modal-container');
    if (!snapshotModalContainer) {
      const container = document.createElement('div');
      container.id = 'snapshot-modal-container';
      document.body.appendChild(container);
      render(html`<${SnapshotPreviewModal} />`, container);
    }
    
    // Video modal
    const videoModalContainer = document.getElementById('video-modal-container');
    if (!videoModalContainer) {
      const container = document.createElement('div');
      container.id = 'video-modal-container';
      document.body.appendChild(container);
      render(html`<${VideoModal} />`, container);
    }
  });
}

/**
 * Setup snapshot modal
 */
export function setupSnapshotModal() {
  // This is now handled by setupModals
  setupModals();
}

/**
 * Add status message styles
 */
export function addStatusMessageStyles() {
  // Check if styles already exist
  if (document.getElementById('status-message-styles')) {
    return;
  }

  // Create style element
  const style = document.createElement('style');
  style.id = 'status-message-styles';

  style.textContent = `
    .status-message {
      position: fixed;
      top: 20px;
      left: 50%;
      transform: translateX(-50%) translateY(-20px);
      background-color: #4CAF50;
      color: white;
      padding: 10px 15px;
      border-radius: 4px;
      z-index: 1000;
      font-size: 14px;
      opacity: 0;
      transition: opacity 0.3s, transform 0.3s;
      max-width: 80%;
      text-align: center;
      box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    }
    
    .status-message.visible {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  `;

  document.head.appendChild(style);
}
