/**
 * LightNVR Web Interface UI Components
 * Preact components for UI elements like modals, status messages, etc.
 */

import { h } from '../../preact.min.js';
import { useState, useEffect, useRef, useCallback } from '../../preact.hooks.module.js';
import { html } from '../../preact-app.js';
import { statusMessageStore, snapshotModalStore, videoModalStore } from '../../preact-app.js';

/**
 * Delete confirmation modal component
 * @param {Object} props - Component props
 * @param {boolean} props.isOpen - Whether the modal is open
 * @param {Function} props.onClose - Function to call when the modal is closed
 * @param {Function} props.onConfirm - Function to call when the delete is confirmed
 * @param {string} props.mode - Delete mode ('selected' or 'all')
 * @param {number} props.count - Number of items to delete (for 'selected' mode)
 * @returns {JSX.Element} Delete confirmation modal component
 */
export function DeleteConfirmationModal({ isOpen, onClose, onConfirm, mode, count }) {
  const getTitle = () => {
    if (mode === 'selected') {
      return `Delete ${count} Selected Recording${count !== 1 ? 's' : ''}`;
    } else {
      return 'Delete All Filtered Recordings';
    }
  };

  const getMessage = () => {
    if (mode === 'selected') {
      return `Are you sure you want to delete ${count} selected recording${count !== 1 ? 's' : ''}? This action cannot be undone.`;
    } else {
      return 'Are you sure you want to delete ALL recordings matching your current filter? This action cannot be undone.';
    }
  };

  const footer = html`
    <button 
      class="px-5 py-2.5 bg-gray-600 text-white rounded-md hover:bg-gray-700 transition-colors shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-opacity-50 font-medium"
      onClick=${onClose}
    >
      Cancel
    </button>
    <button 
      class="px-5 py-2.5 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50 font-medium"
      onClick=${onConfirm}
    >
      Delete
    </button>
  `;
  
  return html`
    <${Modal}
      id="delete-confirmation-modal"
      title=${getTitle()}
      isOpen=${isOpen}
      onClose=${onClose}
      footer=${footer}
    >
      <div class="p-4 text-center">
        <div class="mb-4">
          <svg class="w-16 h-16 mx-auto text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
          </svg>
        </div>
        <p class="text-lg font-medium text-gray-800 dark:text-gray-200 mb-2">Warning: This action cannot be undone</p>
        <p class="text-gray-600 dark:text-gray-400">${getMessage()}</p>
      </div>
    </${Modal}>
  `;
}

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
      class=${`status-message ${state.type === 'error' ? 'error' : ''} ${state.visible ? 'visible' : ''}`}
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
  
  console.log('Modal render called with isOpen:', isOpen);
  
  if (!isOpen) {
    console.log('Modal not rendering because isOpen is false');
    return null;
  }
  
  console.log('Modal rendering with id:', id);
  
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
            ×
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
      console.log('VideoModal: videoModalStore updated', newState);
      setState(newState);
      setIsOpen(!!newState.videoUrl);
      console.log('VideoModal: isOpen set to', !!newState.videoUrl);
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
 * @param {string} type - Message type ('success' or 'error')
 */
export function showStatusMessage(message, duration = 3000, type = 'success') {
  const state = statusMessageStore.getState();
  
  // Clear any existing timeout
  if (state.timeout) {
    clearTimeout(state.timeout);
  }
  
  // Update state
  statusMessageStore.setState({
    message,
    visible: true,
    type,
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
 * @param {string} downloadUrl - Download URL (optional)
 */
export function showVideoModal(videoUrl, title = 'Video Playback', downloadUrl = null) {
  console.log('showVideoModal called with:', { videoUrl, title, downloadUrl });
  
  // If downloadUrl is not provided, generate it from videoUrl
  const finalDownloadUrl = downloadUrl || videoUrl.replace('/play/', '/download/');
  
  console.log('Setting videoModalStore state to:', { videoUrl, title, downloadUrl: finalDownloadUrl });
  
  videoModalStore.setState({
    videoUrl,
    title,
    downloadUrl: finalDownloadUrl
  });
  
  console.log('videoModalStore state after update:', videoModalStore.getState());
}

/**
 * Setup modals
 */
export function setupModals() {
  console.log('setupModals called');
  
  // Render UI components to the DOM
  import('../../preact.min.js').then(({ render }) => {
    console.log('preact.min.js imported');
    
    // Status message
    const statusMessageContainer = document.getElementById('status-message-container');
    console.log('statusMessageContainer:', statusMessageContainer);
    if (!statusMessageContainer) {
      console.log('Creating statusMessageContainer');
      const container = document.createElement('div');
      container.id = 'status-message-container';
      document.body.appendChild(container);
      render(html`<${StatusMessage} />`, container);
      console.log('StatusMessage rendered');
    } else {
      console.log('statusMessageContainer already exists');
      render(html`<${StatusMessage} />`, statusMessageContainer);
      console.log('StatusMessage rendered to existing container');
    }
    
    // Snapshot preview modal
    const snapshotModalContainer = document.getElementById('snapshot-modal-container');
    console.log('snapshotModalContainer:', snapshotModalContainer);
    if (!snapshotModalContainer) {
      console.log('Creating snapshotModalContainer');
      const container = document.createElement('div');
      container.id = 'snapshot-modal-container';
      document.body.appendChild(container);
      render(html`<${SnapshotPreviewModal} />`, container);
      console.log('SnapshotPreviewModal rendered');
    } else {
      console.log('snapshotModalContainer already exists');
      render(html`<${SnapshotPreviewModal} />`, snapshotModalContainer);
      console.log('SnapshotPreviewModal rendered to existing container');
    }
    
    // Video modal
    const videoModalContainer = document.getElementById('video-modal-container');
    console.log('videoModalContainer:', videoModalContainer);
    if (!videoModalContainer) {
      console.log('Creating videoModalContainer');
      const container = document.createElement('div');
      container.id = 'video-modal-container';
      document.body.appendChild(container);
      render(html`<${VideoModal} />`, container);
      console.log('VideoModal rendered');
    } else {
      console.log('videoModalContainer already exists');
      render(html`<${VideoModal} />`, videoModalContainer);
      console.log('VideoModal rendered to existing container');
    }
    
    // Delete confirmation modal container
    const deleteModalContainer = document.getElementById('delete-modal-container');
    if (!deleteModalContainer) {
      const container = document.createElement('div');
      container.id = 'delete-modal-container';
      document.body.appendChild(container);
    }
  }).catch(error => {
    console.error('Error importing preact.min.js:', error);
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
    
    .status-message.error {
      background-color: #F44336;
    }
    
    .status-message.visible {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
    
    /* Modal styles */
    .modal {
      display: flex;
      position: fixed;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.5);
      z-index: 1000;
      overflow: auto;
      align-items: center;
      justify-content: center;
    }
    
    .modal-content {
      background-color: white;
      margin: 2rem auto;
      width: 100%;
      max-width: 600px;
      border-radius: 8px;
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
      animation: modalAppear 0.3s;
    }
    
    @keyframes modalAppear {
      from {
        opacity: 0;
        transform: translateY(-50px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }
    
    .modal-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 1rem;
      border-bottom: 1px solid #e0e0e0;
    }
    
    .modal-header h3 {
      margin: 0;
    }
    
    .modal-body {
      padding: 1rem;
      max-height: 70vh;
      overflow-y: auto;
    }
    
    .modal-footer {
      padding: 1rem;
      display: flex;
      justify-content: flex-end;
      gap: 0.5rem;
      border-top: 1px solid #e0e0e0;
    }
    
    .close {
      font-size: 1.5rem;
      font-weight: bold;
      cursor: pointer;
      color: #9e9e9e;
    }
    
    .close:hover {
      color: #f44336;
    }
  `;

  document.head.appendChild(style);
}

/**
 * Add modal styles
 */
export function addModalStyles() {
  // Check if styles already exist
  if (document.getElementById('modal-styles')) {
    return;
  }

  // Create style element
  const style = document.createElement('style');
  style.id = 'modal-styles';

  style.textContent = `
    /* Modal styles */
    .modal {
      display: flex !important;
      position: fixed;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.5);
      z-index: 1000;
      overflow: auto;
      align-items: center;
      justify-content: center;
    }
    
    .modal-content {
      background-color: white;
      margin: 2rem auto;
      width: 100%;
      max-width: 600px;
      border-radius: 8px;
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
      animation: modalAppear 0.3s;
    }
    
    @keyframes modalAppear {
      from {
        opacity: 0;
        transform: translateY(-50px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }
    
    .modal-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 1rem;
      border-bottom: 1px solid #e0e0e0;
    }
    
    .modal-header h3 {
      margin: 0;
    }
    
    .modal-body {
      padding: 1rem;
      max-height: 70vh;
      overflow-y: auto;
    }
    
    .modal-footer {
      padding: 1rem;
      display: flex;
      justify-content: flex-end;
      gap: 0.5rem;
      border-top: 1px solid #e0e0e0;
    }
    
    .close {
      font-size: 1.5rem;
      font-weight: bold;
      cursor: pointer;
      color: #9e9e9e;
    }
    
    .close:hover {
      color: #f44336;
    }
  `;

  document.head.appendChild(style);
}
