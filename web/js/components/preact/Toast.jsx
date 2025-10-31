/**
 * LightNVR Toast Notification System
 * JSX-based toast notifications using Preact
 */
import { h, render, Component, createContext } from 'preact';
import { useState, useEffect, useContext } from 'preact/hooks';

// Create a context for the toast system
const ToastContext = createContext(null);

// Toast colors - using CSS variables for theme support
const getToastColor = (type) => {
  const colorMap = {
    success: 'var(--success)',
    error: 'var(--danger)',
    warning: 'var(--warning)',
    info: 'var(--info)'
  };
  return `hsl(${colorMap[type] || colorMap.info})`;
};

const getToastForegroundColor = (type) => {
  const colorMap = {
    success: 'var(--success-foreground)',
    error: 'var(--danger-foreground)',
    warning: 'var(--warning-foreground)',
    info: 'var(--info-foreground)'
  };
  return `hsl(${colorMap[type] || colorMap.info})`;
};

// Toast icons (Unicode symbols)
const toastIcons = {
  success: '✓',
  error: '✕',
  warning: '⚠',
  info: 'ℹ'
};

const containerStyle = {
  position: 'fixed',
  top: '20px',
  left: '50%',
  transform: 'translateX(-50%)',
  zIndex: '10000',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  pointerEvents: 'none' // Allow clicking through the container
};

/**
 * Individual Toast Component
 */
const Toast = ({ id, message, type, onRemove }) => {
  const [isVisible, setIsVisible] = useState(false);
  const [isExiting, setIsExiting] = useState(false);
  
  useEffect(() => {
    // Trigger entrance animation after component mounts
    requestAnimationFrame(() => setIsVisible(true));
    
    return () => setIsVisible(false);
  }, []);
  
  const handleRemove = () => {
    setIsExiting(true);
    // Wait for exit animation to complete before removing from DOM
    setTimeout(() => {
      onRemove();
    }, 300); // Match transition duration
  };
  
  const toastStyle = {
    padding: '10px 15px',
    borderRadius: '4px',
    marginBottom: '10px',
    boxShadow: '0 3px 10px rgba(0,0,0,0.2)',
    minWidth: '250px',
    textAlign: 'left',
    color: getToastForegroundColor(type),
    backgroundColor: getToastColor(type),
    opacity: isExiting ? 0 : (isVisible ? 1 : 0),
    transform: isExiting ? 'translateY(-20px)' : (isVisible ? 'translateY(0)' : 'translateY(-20px)'),
    transition: 'opacity 0.3s ease, transform 0.3s ease',
    position: 'relative',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    pointerEvents: 'auto', // Make the toast clickable
    border: '1px solid rgba(255,255,255,0.1)'
  };
  
  const closeButtonStyle = {
    marginLeft: '10px',
    background: 'none',
    border: 'none',
    color: getToastForegroundColor(type),
    fontSize: '16px',
    cursor: 'pointer',
    opacity: '0.7',
    transition: 'opacity 0.2s ease',
    padding: '0 5px',
    fontWeight: 'bold'
  };
  
  const iconStyle = {
    marginRight: '10px',
    fontSize: '16px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: '20px',
    height: '20px'
  };
  
  const contentStyle = {
    flex: 1,
    display: 'flex',
    alignItems: 'center'
  };
  
  const messageStyle = {
    marginLeft: '5px'
  };
  
  return (
    <div style={toastStyle} data-toast-id={id}>
      <div style={contentStyle}>
        <span style={iconStyle}>{toastIcons[type] || toastIcons.info}</span>
        <span style={messageStyle}>{message}</span>
      </div>
      <button 
        style={closeButtonStyle} 
        onClick={handleRemove}
        onMouseOver={(e) => e.currentTarget.style.opacity = '1'}
        onMouseOut={(e) => e.currentTarget.style.opacity = '0.7'}
      >
        ×
      </button>
    </div>
  );
};

/**
 * Toast Container Component
 */
class ToastContainer extends Component {
  state = {
    toasts: []
  };
  
  addToast = (message, type, duration) => {
    const id = Date.now();
    const newToast = { id, message, type, duration };
    
    this.setState(prevState => ({
      toasts: [...prevState.toasts, newToast]
    }));
    
    // Set timeout to auto-remove the toast after duration
    if (duration > 0) {
      setTimeout(() => {
        // Find the toast in the current state
        const toast = this.state.toasts.find(t => t.id === id);
        if (toast) {
          // Mark toast for exit animation
          this.setState(prevState => ({
            toasts: prevState.toasts.map(t => 
              t.id === id ? { ...t, exiting: true } : t
            )
          }));
          
          // Remove after animation completes
          setTimeout(() => this.removeToast(id), 300);
        }
      }, duration - 300); // Start animation before duration ends
    }
    
    return id;
  };
  
  removeToast = (id) => {
    this.setState(prevState => ({
      toasts: prevState.toasts.filter(toast => toast.id !== id)
    }));
  };
  
  render() {
    const { toasts } = this.state;
    
    if (toasts.length === 0) return null;
    
    return (
      <div id="toast-container" style={containerStyle}>
        {toasts.map(toast => (
          <Toast
            key={toast.id}
            id={toast.id}
            message={toast.message}
            type={toast.type}
            onRemove={() => this.removeToast(toast.id)}
          />
        ))}
      </div>
    );
  }
}

/**
 * Toast Provider Component
 * This component provides the toast functionality to the entire app
 */
export class ToastProvider extends Component {
  constructor(props) {
    super(props);
    this.toastContainerRef = null;
  }
  
  componentDidMount() {
    // Initialize the toast container when the provider mounts
    if (!this.toastContainerRef) {
      // Create a container element for the toast container
      const containerElement = document.createElement('div');
      containerElement.id = 'toast-container-root';
      document.body.appendChild(containerElement);
      
      // Render the ToastContainer into the DOM
      this.toastContainerRef = render(<ToastContainer />, containerElement);
      
      console.log('Toast container initialized');
    }
  }
  
  componentWillUnmount() {
    // Clean up the toast container when the provider unmounts
    if (this.toastContainerRef) {
      // Remove the container element from the DOM
      const containerElement = document.getElementById('toast-container-root');
      if (containerElement) {
        document.body.removeChild(containerElement);
      }
      
      this.toastContainerRef = null;
      console.log('Toast container cleaned up');
    }
  }
  
  // Methods to show different types of toasts
  showToast = (message, type = 'info', duration = 4000) => {
    if (this.toastContainerRef) {
      return this.toastContainerRef.addToast(message, type, duration);
    }
    console.error('Toast container not initialized');
    return null;
  };
  
  showSuccessToast = (message, duration = 4000) => {
    return this.showToast(message, 'success', duration);
  };
  
  showErrorToast = (message, duration = 4000) => {
    return this.showToast(message, 'error', duration);
  };
  
  showWarningToast = (message, duration = 4000) => {
    return this.showToast(message, 'warning', duration);
  };
  
  showInfoToast = (message, duration = 4000) => {
    return this.showToast(message, 'info', duration);
  };
  
  render() {
    const contextValue = {
      showToast: this.showToast,
      showSuccessToast: this.showSuccessToast,
      showErrorToast: this.showErrorToast,
      showWarningToast: this.showWarningToast,
      showInfoToast: this.showInfoToast
    };
    
    return (
      <ToastContext.Provider value={contextValue}>
        {this.props.children}
      </ToastContext.Provider>
    );
  }
}

/**
 * Hook to use the toast functionality in functional components
 */
export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context;
}

// Singleton instance for use outside of React components
let toastInstance = null;

/**
 * Initialize the toast system
 * This should be called once at app startup
 */
export function initToasts() {
  if (!toastInstance) {
    // Create a container element for the toast container
    const containerElement = document.createElement('div');
    containerElement.id = 'toast-container-root';
    document.body.appendChild(containerElement);
    
    // Render the ToastContainer into the DOM
    toastInstance = render(<ToastContainer />, containerElement);
    
    console.log('Toast system initialized');
  }
  return toastInstance;
}

// Export functions for use outside of React components
export function showToast(message, type = 'info', duration = 4000) {
  const instance = toastInstance || initToasts();
  return instance.addToast(message, type, duration);
}

export function showSuccessToast(message, duration = 4000) {
  return showToast(message, 'success', duration);
}

export function showErrorToast(message, duration = 4000) {
  return showToast(message, 'error', duration);
}

export function showWarningToast(message, duration = 4000) {
  return showToast(message, 'warning', duration);
}

export function showInfoToast(message, duration = 4000) {
  return showToast(message, 'info', duration);
}

// For backward compatibility
export function createDirectToast(message, type = 'info', duration = 4000) {
  return showToast(message, type, duration);
}

export function showStatusMessage(message, type = 'info', duration = 4000) {
  // Map type strings to the new format
  let toastType = 'info';
  
  if (type === 'success') {
    toastType = 'success';
  } else if (type === 'error') {
    toastType = 'error';
  } else if (type === 'warning') {
    toastType = 'warning';
  }
  
  return showToast(message, toastType, duration);
}

// Export for global use
if (typeof window !== 'undefined') {
  // Initialize the toast system
  initToasts();
  
  // Make toast functions available globally
  window.showSuccessToast = showSuccessToast;
  window.showErrorToast = showErrorToast;
  window.showWarningToast = showWarningToast;
  window.showInfoToast = showInfoToast;
  window.showToast = showToast;
  window.showStatusMessage = showStatusMessage;
  window.createDirectToast = createDirectToast;
  
  // Log for debugging
  console.log('Toast functions exported to window object');
  
  // Add a test function to the window object
  window.testToast = (type = 'info') => {
    const message = `Test ${type} toast at ${new Date().toLocaleTimeString()}`;
    console.log(`Triggering test toast: ${message}`);
    showToast(message, type);
    console.log(`Test toast triggered: ${message}`);
    return 'Toast triggered';
  };
}
